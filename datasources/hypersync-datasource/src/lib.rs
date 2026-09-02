//! HyperSync datasource: bounded historical backfill with server-side
//! filtering.
//!
//! [HyperSync](https://docs.envio.dev/docs/HyperSync/overview) is Envio's
//! columnar Solana archive. Unlike the RPC crawlers and the Jetstreamer
//! archive reader, which download every transaction in the range and filter
//! client-side, HyperSync pushes the program filter into the server, so only
//! the matched transactions cross the network. For a program that appears in
//! a few percent of transactions this reduces bytes moved by one to two
//! orders of magnitude.
//!
//! The datasource is bounded, like `carbon-jetstreamer-datasource`: give it a
//! `[from_slot, to_slot)` range and a set of program ids, and it runs the
//! range to completion, then lets the pipeline shut down cleanly.
//!
//! # How it fetches
//!
//! Two queries per page, so that decoders see complete transactions:
//!
//! 1. Filter `instruction_calls` by `executing_account` (the program ids) and
//!    collect the matching transaction ids for the page's slot window.
//! 2. Re-query those transaction ids with no instruction filter. HyperSync
//!    hydrates every instruction of each matched transaction, plus the
//!    transaction row (account keys, address-lookup-table expansions,
//!    signatures, fee) and the block row (blockhash, block time).
//!
//! Each transaction is then reassembled into a `TransactionUpdate`: compiled
//! instructions are rebuilt from the stored per-instruction account index
//! lists, and inner instructions are regrouped from the stored CPI paths with
//! `stack_height = path.len()`, so `carbon-core`'s nesting reconstruction
//! reproduces the original instruction tree exactly.
//!
//! # Fidelity limits, stated plainly
//!
//! - **History window.** The public endpoint's historical depth is a moving
//!   floor: Envio is actively growing the backfill based on user demand, so
//!   check the endpoint for current coverage. Below the floor, compose with
//!   `carbon-jetstreamer-datasource`, which reaches back to genesis but lags
//!   the head by about two epochs; this datasource serves the recent window
//!   Jetstreamer cannot.
//! - **Failed transactions are not served.** The archive stores no
//!   instruction rows for them, so this datasource only emits successful
//!   transactions. (`carbon-rpc-block-crawler-datasource` also skips failed
//!   transactions, so pipelines behave identically across the two.)
//! - **Vote transactions are excluded at ingest**, and
//!   `TransactionUpdate.index` is therefore the dense rank over non-vote
//!   transactions of the slot, not the original block position.
//! - **The message header is synthesized.** `num_required_signatures` is
//!   recovered from the signature count; the readonly counts are set to zero,
//!   so `is_writable` / `is_signer` on `AccountMeta` are approximations. No
//!   decoder in this repository reads them (verified against all 63 decoder
//!   crates); processors that need exact flags should not rely on them.
//! - `meta.log_messages`, `meta.pre/post_balances`, token balances,
//!   `return_data` and per-transaction rewards are not populated in v1.
//!   `recent_blockhash` is populated when the archive serves it and is
//!   otherwise `Hash::default()`. (`carbon-jetstreamer-datasource` similarly
//!   leaves `block_time` and `block_hash` unset; both are populated here.)

use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{Datasource, DatasourceId, TransactionUpdate, Update, UpdateType},
        error::{CarbonResult, Error},
    },
    hypersync_client_solana::{
        config::ClientConfig,
        simple_types::{InstructionCall, SolanaResponse, Transaction as HsTransaction},
        Client,
    },
    hypersync_solana_net_types::{
        field_selection::{BlockField, InstructionField, SolanaFieldSelection, TransactionField},
        query::{InstructionSelection, SolanaQuery, TransactionSelection},
        Address as HsAddress, Signature as HsSignature,
    },
    solana_hash::Hash,
    solana_message::{
        compiled_instruction::CompiledInstruction,
        v0::{LoadedAddresses, Message as MessageV0},
        MessageHeader, VersionedMessage,
    },
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_transaction::versioned::VersionedTransaction,
    solana_transaction_status::{InnerInstruction, InnerInstructions, TransactionStatusMeta},
    std::{
        collections::{BTreeMap, HashMap},
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc,
        },
        time::Duration,
    },
    tokio::sync::mpsc::Sender,
    tokio_util::sync::CancellationToken,
};

/// Envio's public Solana mainnet archive endpoint. Requires a bearer token
/// (free at <https://envio.dev>).
pub const MAINNET_HISTORY_URL: &str = "https://solana-mainnet-history.hypersync.xyz";

/// Solana's maximum CPI depth; instruction paths deeper than this are
/// malformed and skipped rather than fed into `carbon-core`'s fixed-size
/// nesting stack.
const MAX_STACK_DEPTH: usize = 5;

/// How many transaction ids to put in one hydration request.
const ID_CHUNK: usize = 1_000;

/// Consecutive non-advancing pages tolerated before erroring out.
const MAX_STALLS: u32 = 5;

/// Counters the datasource increments while it runs; read them after the
/// pipeline finishes for throughput accounting.
#[derive(Debug, Default)]
pub struct HyperSyncStats {
    /// Completed slot pages (pass 1 windows).
    pub pages: AtomicU64,
    /// HTTP queries issued (both passes).
    pub requests: AtomicU64,
    /// Sum of raw response bytes across all queries.
    pub response_bytes: AtomicU64,
    /// `TransactionUpdate`s emitted.
    pub transactions: AtomicU64,
    /// Instruction rows contained in emitted transactions (top-level + inner).
    pub instructions: AtomicU64,
}

/// Bounded historical backfill over a HyperSync archive, filtered server-side
/// by program id.
pub struct HyperSyncDatasource {
    pub url: String,
    pub bearer_token: Option<String>,
    /// Inclusive start slot.
    pub from_slot: u64,
    /// Exclusive end slot.
    pub to_slot: u64,
    /// Emit every transaction containing at least one instruction (top-level
    /// or CPI) executed by one of these programs.
    pub programs: Vec<Pubkey>,
    /// Cap on matched transactions per page, which bounds the id list carried
    /// into the hydration pass.
    pub max_transactions_per_page: usize,
    /// Number of concurrent workers, each owning a contiguous stripe of the
    /// slot range. Like `carbon-jetstreamer-datasource`'s `threads`, going
    /// above 1 gives up cross-stripe delivery order, which carbon-core does
    /// not guarantee anyway (updates from concurrent datasources interleave).
    pub concurrency: usize,
    stats: Arc<HyperSyncStats>,
}

impl HyperSyncDatasource {
    pub fn new(
        url: String,
        bearer_token: Option<String>,
        from_slot: u64,
        to_slot: u64,
        programs: Vec<Pubkey>,
    ) -> Self {
        Self {
            url,
            bearer_token,
            from_slot,
            to_slot,
            programs,
            max_transactions_per_page: 2_000,
            concurrency: 10,
            stats: Arc::new(HyperSyncStats::default()),
        }
    }

    /// Convenience constructor against Envio's public mainnet archive,
    /// mirroring `JetstreamerDatasource::new_with_old_faithful_mainnet`.
    pub fn new_mainnet_history(
        bearer_token: Option<String>,
        from_slot: u64,
        to_slot: u64,
        programs: Vec<Pubkey>,
    ) -> Self {
        Self::new(
            MAINNET_HISTORY_URL.to_string(),
            bearer_token,
            from_slot,
            to_slot,
            programs,
        )
    }

    /// Handle to the run counters; clone before moving the datasource into
    /// the pipeline.
    pub fn stats(&self) -> Arc<HyperSyncStats> {
        Arc::clone(&self.stats)
    }

    fn client(&self) -> CarbonResult<Client> {
        Client::new(ClientConfig {
            url: self.url.clone(),
            bearer_token: self.bearer_token.clone(),
            ..ClientConfig::default()
        })
        .map_err(|e| Error::Custom(format!("hypersync client: {e}")))
    }
}

/// Shared context for the range workers: one HTTP client (cheap to clone,
/// shares its rate-limit state), the resolved program filter, and the run
/// counters.
struct StripeContext {
    client: Client,
    programs: Vec<HsAddress>,
    max_transactions_per_page: usize,
    stats: Arc<HyperSyncStats>,
}

impl StripeContext {
    async fn get(&self, query: &SolanaQuery) -> CarbonResult<SolanaResponse> {
        let response = self
            .client
            .get(query)
            .await
            .map_err(|e| Error::Custom(format!("hypersync query: {e}")))?;
        self.stats.requests.fetch_add(1, Ordering::Relaxed);
        self.stats
            .response_bytes
            .fetch_add(response.response_bytes as u64, Ordering::Relaxed);
        Ok(response)
    }

    /// Pass 1: which transactions in `[cursor, to)` touch the programs.
    /// Returns `(ids, next_slot)`.
    async fn matched_ids(&self, cursor: u64, to: u64) -> CarbonResult<(Vec<HsSignature>, u64)> {
        let query = SolanaQuery {
            from_slot: cursor,
            to_slot: Some(to),
            instruction_calls: vec![InstructionSelection {
                executing_account: self.programs.clone(),
                ..Default::default()
            }],
            field_selection: SolanaFieldSelection {
                // Slim both tables: identity column only on the filtered
                // table, identity + id on the hydrated transactions.
                instruction_call: vec![InstructionField::Slot],
                transaction: vec![
                    TransactionField::Slot,
                    TransactionField::TransactionIndex,
                    TransactionField::TransactionId,
                ],
                ..Default::default()
            },
            max_num_transactions: Some(self.max_transactions_per_page),
            ..Default::default()
        };
        let response = self.get(&query).await?;
        let ids = response
            .transactions
            .iter()
            .filter_map(|t| t.transaction_id)
            .collect();
        Ok((ids, response.next_slot))
    }

    /// Pass 2: hydrate every instruction of the matched transactions in
    /// `[from, to)`, plus transaction and block rows.
    async fn hydrate(&self, from: u64, to: u64, ids: &[HsSignature]) -> CarbonResult<Bundles> {
        let mut bundles = Bundles::default();
        for chunk in ids.chunks(ID_CHUNK) {
            let mut cursor = from;
            while cursor < to {
                let query = SolanaQuery {
                    from_slot: cursor,
                    to_slot: Some(to),
                    transactions: vec![TransactionSelection {
                        transaction_id: chunk.to_vec(),
                        ..Default::default()
                    }],
                    field_selection: SolanaFieldSelection {
                        instruction_call: vec![
                            InstructionField::Slot,
                            InstructionField::TransactionIndex,
                            InstructionField::InstructionAddress,
                            InstructionField::ExecutingAccount,
                            InstructionField::ExecutingAccountIndex,
                            InstructionField::AccountArguments,
                            InstructionField::AccountIndexArguments,
                            InstructionField::Data,
                        ],
                        transaction: vec![
                            TransactionField::Slot,
                            TransactionField::TransactionIndex,
                            TransactionField::TransactionId,
                            TransactionField::Signatures,
                            TransactionField::FeePayer,
                            TransactionField::Fee,
                            TransactionField::ComputeUnitsConsumed,
                            TransactionField::AccountKeys,
                            TransactionField::RecentBlockhash,
                            TransactionField::LoadedAddressesWritable,
                            TransactionField::LoadedAddressesReadonly,
                        ],
                        block: vec![
                            BlockField::Slot,
                            BlockField::Blockhash,
                            BlockField::BlockTime,
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let response = self.get(&query).await?;
                bundles.absorb(response.transactions, response.instruction_calls);
                for block in &response.blocks {
                    if let Some(slot) = block.slot {
                        bundles
                            .blocks
                            .insert(slot, (block.blockhash, block.block_time));
                    }
                }
                if response.next_slot <= cursor {
                    return Err(Error::Custom(format!(
                        "hydration cursor stalled at slot {cursor}"
                    )));
                }
                cursor = response.next_slot;
            }
        }
        Ok(bundles)
    }
}

/// Rows of one hydration window, grouped for assembly.
#[derive(Default)]
struct Bundles {
    transactions: BTreeMap<(u64, u32), HsTransaction>,
    instructions: BTreeMap<(u64, u32), Vec<InstructionCall>>,
    blocks: HashMap<u64, (Option<hypersync_solana_net_types::Hash>, Option<i64>)>,
}

impl Bundles {
    fn absorb(&mut self, transactions: Vec<HsTransaction>, instructions: Vec<InstructionCall>) {
        for tx in transactions {
            if let (Some(slot), Some(index)) = (tx.slot, tx.transaction_index) {
                self.transactions.insert((slot, index), tx);
            }
        }
        for row in instructions {
            if let (Some(slot), Some(index)) = (row.slot, row.transaction_index) {
                self.instructions
                    .entry((slot, index))
                    .or_default()
                    .push(row);
            }
        }
    }
}

/// One worker's page loop over `[from, to)`.
async fn run_stripe(
    context: Arc<StripeContext>,
    from: u64,
    to: u64,
    id: DatasourceId,
    sender: Sender<(Update, DatasourceId)>,
    cancellation_token: CancellationToken,
) -> CarbonResult<()> {
    let mut cursor = from;
    let mut stalls = 0u32;

    while cursor < to {
        if cancellation_token.is_cancelled() {
            log::info!("hypersync stripe cancelled at slot {cursor}");
            return Ok(());
        }

        let (ids, next_slot) = context.matched_ids(cursor, to).await?;
        if next_slot <= cursor {
            stalls += 1;
            if stalls >= MAX_STALLS {
                return Err(Error::Custom(format!(
                    "cursor stalled at slot {cursor} after {MAX_STALLS} attempts \
                     (range beyond the endpoint's queryable tip?)"
                )));
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
            continue;
        }
        stalls = 0;

        if !ids.is_empty() {
            let Bundles {
                transactions,
                mut instructions,
                blocks,
            } = context.hydrate(cursor, next_slot, &ids).await?;
            for (key, tx) in &transactions {
                let rows = instructions.remove(key).unwrap_or_default();
                let block = blocks.get(&key.0).copied().unwrap_or((None, None));
                match assemble(tx, rows, block) {
                    Ok((update, instruction_count)) => {
                        context.stats.transactions.fetch_add(1, Ordering::Relaxed);
                        context
                            .stats
                            .instructions
                            .fetch_add(instruction_count, Ordering::Relaxed);
                        if sender
                            .send((Update::Transaction(Box::new(update)), id.clone()))
                            .await
                            .is_err()
                        {
                            log::info!("update channel closed, stopping hypersync stripe");
                            return Ok(());
                        }
                    }
                    Err(reason) => {
                        // Skip loudly: a malformed transaction should never
                        // take the pipeline down, but must not vanish
                        // silently either.
                        log::warn!(
                            "skipping transaction at slot {} index {}: {reason}",
                            key.0,
                            key.1
                        );
                    }
                }
            }
        }

        context.stats.pages.fetch_add(1, Ordering::Relaxed);
        cursor = next_slot;
    }

    log::debug!("hypersync stripe finished range {from}..{to}");
    Ok(())
}

#[async_trait]
impl Datasource for HyperSyncDatasource {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
    ) -> CarbonResult<()> {
        let context = Arc::new(StripeContext {
            client: self.client()?,
            programs: self
                .programs
                .iter()
                .map(|p| HsAddress(p.to_bytes()))
                .collect(),
            max_transactions_per_page: self.max_transactions_per_page,
            stats: Arc::clone(&self.stats),
        });

        let total = self.to_slot.saturating_sub(self.from_slot);
        let workers = (self.concurrency.max(1) as u64).min(total.max(1));
        let stripe = total / workers;
        let remainder = total % workers;

        let mut handles = Vec::with_capacity(workers as usize);
        let mut start = self.from_slot;
        for w in 0..workers {
            let len = stripe + u64::from(w < remainder);
            let end = start + len;
            handles.push(tokio::spawn(run_stripe(
                Arc::clone(&context),
                start,
                end,
                id.clone(),
                sender.clone(),
                cancellation_token.clone(),
            )));
            start = end;
        }
        drop(sender);

        for handle in handles {
            handle
                .await
                .map_err(|e| Error::Custom(format!("hypersync worker panicked: {e}")))??;
        }

        log::info!(
            "hypersync datasource finished range {}..{}",
            self.from_slot,
            self.to_slot
        );
        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![UpdateType::Transaction]
    }
}

/// Rebuild a `TransactionUpdate` from HyperSync rows. Returns the update and
/// the number of instruction rows it carries.
fn assemble(
    tx: &HsTransaction,
    mut rows: Vec<InstructionCall>,
    block: (Option<hypersync_solana_net_types::Hash>, Option<i64>),
) -> Result<(TransactionUpdate, u64), String> {
    let slot = tx.slot.ok_or("transaction row missing slot")?;
    let transaction_index = tx
        .transaction_index
        .ok_or("transaction row missing index")?;

    let signatures: Vec<Signature> = tx
        .signatures
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|s| Signature::from(s.0))
        .collect();
    if signatures.is_empty() {
        return Err("no signatures".into());
    }

    let static_keys: Vec<Pubkey> = tx
        .account_keys
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|a| Pubkey::new_from_array(a.0))
        .collect();
    if static_keys.is_empty() {
        return Err("no account keys".into());
    }
    let loaded_writable: Vec<Pubkey> = tx
        .loaded_addresses_writable
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|a| Pubkey::new_from_array(a.0))
        .collect();
    let loaded_readonly: Vec<Pubkey> = tx
        .loaded_addresses_readonly
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|a| Pubkey::new_from_array(a.0))
        .collect();

    // The resolved key list, in the exact order carbon-core's transformer
    // rebuilds it: static keys, then ALT writable, then ALT readonly.
    let mut resolved = static_keys.clone();
    resolved.extend_from_slice(&loaded_writable);
    resolved.extend_from_slice(&loaded_readonly);
    let position_of: HashMap<Pubkey, u8> = resolved
        .iter()
        .enumerate()
        .filter(|(i, _)| *i <= u8::MAX as usize)
        .map(|(i, k)| (*k, i as u8))
        .collect();

    rows.sort_by(|a, b| a.instruction_address.cmp(&b.instruction_address));

    let mut top_level: Vec<CompiledInstruction> = Vec::new();
    let mut inner_groups: BTreeMap<u8, Vec<InnerInstruction>> = BTreeMap::new();
    let mut instruction_count = 0u64;

    for row in &rows {
        let path = row
            .instruction_address
            .as_deref()
            .ok_or("instruction row missing address path")?;
        if path.is_empty() {
            return Err("empty instruction path".into());
        }
        if path.len() > MAX_STACK_DEPTH {
            return Err(format!("instruction path depth {} exceeds 5", path.len()));
        }
        let compiled = compile(row, resolved.len(), &position_of)?;
        instruction_count += 1;
        if path.len() == 1 {
            // The archive stores every executed top-level instruction of a
            // successful transaction, so positions must come out contiguous.
            if path[0] as usize != top_level.len() {
                return Err(format!(
                    "non-contiguous top-level instructions: got {} expected {}",
                    path[0],
                    top_level.len()
                ));
            }
            top_level.push(compiled);
        } else {
            let index = u8::try_from(path[0]).map_err(|_| "top-level index over 255")?;
            inner_groups
                .entry(index)
                .or_default()
                .push(InnerInstruction {
                    instruction: compiled,
                    stack_height: Some(path.len() as u32),
                });
        }
    }
    if top_level.is_empty() {
        return Err("no top-level instructions".into());
    }

    let message = VersionedMessage::V0(MessageV0 {
        header: MessageHeader {
            num_required_signatures: signatures.len().min(u8::MAX as usize) as u8,
            // Not recoverable from the archive; see the module docs.
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: static_keys,
        recent_blockhash: tx
            .recent_blockhash
            .map(|h| Hash::new_from_array(h.0))
            .unwrap_or_default(),
        instructions: top_level,
        // Original table references are not stored; the loaded addresses are
        // carried in `meta.loaded_addresses`, which is where carbon-core's
        // transformer reads them from.
        address_table_lookups: vec![],
    });

    let meta = TransactionStatusMeta {
        // Only successful transactions exist in the archive.
        status: Ok(()),
        fee: tx.fee.unwrap_or_default(),
        pre_balances: vec![],
        post_balances: vec![],
        inner_instructions: Some(
            inner_groups
                .into_iter()
                .map(|(index, instructions)| InnerInstructions {
                    index,
                    instructions,
                })
                .collect(),
        ),
        log_messages: None,
        pre_token_balances: None,
        post_token_balances: None,
        rewards: None,
        loaded_addresses: LoadedAddresses {
            writable: loaded_writable,
            readonly: loaded_readonly,
        },
        return_data: None,
        compute_units_consumed: tx.compute_units_consumed,
        cost_units: None,
    };

    let update = TransactionUpdate {
        signature: signatures[0],
        transaction: VersionedTransaction {
            signatures,
            message,
        },
        meta,
        is_vote: false,
        slot,
        index: Some(transaction_index as u64),
        block_time: block.1,
        block_hash: block.0.map(|h| Hash::new_from_array(h.0)),
    };
    Ok((update, instruction_count))
}

/// One instruction row into a `CompiledInstruction`, preferring the stored
/// index columns and falling back to position lookup by address.
fn compile(
    row: &InstructionCall,
    resolved_len: usize,
    position_of: &HashMap<Pubkey, u8>,
) -> Result<CompiledInstruction, String> {
    let program_id_index = match row.executing_account_index {
        Some(index) => u8::try_from(index).map_err(|_| "program index over 255")?,
        None => {
            let program = row.executing_account.ok_or("missing executing account")?;
            *position_of
                .get(&Pubkey::new_from_array(program.0))
                .ok_or("executing account not in key list")?
        }
    };
    if (program_id_index as usize) >= resolved_len {
        return Err(format!(
            "program index {program_id_index} out of bounds ({resolved_len} keys)"
        ));
    }

    let accounts: Vec<u8> = match row.account_index_arguments.as_deref() {
        Some(indices) => indices
            .iter()
            .map(|&i| u8::try_from(i).map_err(|_| "account index over 255".to_string()))
            .collect::<Result<_, _>>()?,
        None => row
            .account_arguments
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|a| {
                position_of
                    .get(&Pubkey::new_from_array(a.0))
                    .copied()
                    .ok_or_else(|| "account argument not in key list".to_string())
            })
            .collect::<Result<_, _>>()?,
    };
    if let Some(&max) = accounts.iter().max() {
        if (max as usize) >= resolved_len {
            return Err(format!(
                "account index {max} out of bounds ({resolved_len} keys)"
            ));
        }
    }

    Ok(CompiledInstruction {
        program_id_index,
        accounts,
        data: row.data.clone().unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use {super::*, carbon_core::transaction::TransactionMetadata};

    fn addr(tag: u8) -> HsAddress {
        let mut bytes = [0u8; 32];
        bytes[0] = tag;
        HsAddress(bytes)
    }

    fn row(path: &[u32], program_index: u32, accounts: &[u32], data: &[u8]) -> InstructionCall {
        InstructionCall {
            slot: Some(430_000_000),
            transaction_index: Some(3),
            instruction_address: Some(path.to_vec()),
            executing_account: Some(addr(program_index as u8)),
            executing_account_index: Some(program_index),
            account_arguments: Some(accounts.iter().map(|&i| addr(i as u8)).collect()),
            account_index_arguments: Some(accounts.to_vec()),
            data: Some(data.to_vec()),
            ..Default::default()
        }
    }

    /// Six static keys (tags 0..=5) plus one ALT-writable (6) and one
    /// ALT-readonly (7), so resolved positions 6 and 7 exercise the lookup
    /// expansion.
    fn transaction() -> HsTransaction {
        HsTransaction {
            slot: Some(430_000_000),
            transaction_index: Some(3),
            signatures: Some(vec![HsSignature([9u8; 64])]),
            fee_payer: Some(addr(0)),
            fee: Some(5_000),
            compute_units_consumed: Some(120_000),
            account_keys: Some((0..6).map(addr).collect()),
            loaded_addresses_writable: Some(vec![addr(6)]),
            loaded_addresses_readonly: Some(vec![addr(7)]),
            ..Default::default()
        }
    }

    /// The load-bearing property: paths we emit survive carbon-core's own
    /// counter-walk reconstruction byte for byte, including a depth-3 chain
    /// and an ALT-resolved account index.
    #[test]
    fn cpi_paths_round_trip_through_carbon_core() {
        let paths: Vec<Vec<u32>> = vec![
            vec![0],
            vec![1],
            vec![1, 0],
            vec![1, 1],
            vec![1, 1, 0],
            vec![2],
        ];
        let rows: Vec<InstructionCall> = paths
            .iter()
            .map(|p| row(p, 5, &[1, 2, 6, 7], &[0xAB, 1, 2]))
            .collect();

        let (update, count) = assemble(&transaction(), rows, (None, Some(1_234))).unwrap();
        assert_eq!(count, 6);
        assert_eq!(update.slot, 430_000_000);
        assert_eq!(update.index, Some(3));
        assert_eq!(update.block_time, Some(1_234));
        assert!(!update.is_vote);
        assert!(update.meta.status.is_ok());

        let metadata = std::sync::Arc::new(TransactionMetadata::try_from(update.clone()).unwrap());
        let extracted =
            carbon_core::transformers::extract_instructions_with_metadata(&metadata, &update)
                .unwrap();

        let reconstructed: Vec<Vec<u32>> = extracted
            .iter()
            .map(|(meta, _)| meta.absolute_path.iter().map(|&b| u32::from(b)).collect())
            .collect();
        assert_eq!(reconstructed, paths);

        // Stack heights equal path depth, and the ALT accounts resolved to
        // the positions carbon-core's key concatenation produces.
        for ((meta, instruction), path) in extracted.iter().zip(&paths) {
            assert_eq!(meta.stack_height as usize, path.len());
            assert_eq!(instruction.accounts.len(), 4);
            assert_eq!(
                instruction.accounts[2].pubkey,
                Pubkey::new_from_array({
                    let mut b = [0u8; 32];
                    b[0] = 6;
                    b
                })
            );
        }
    }

    /// With the index columns absent, compilation falls back to resolving
    /// positions from the addresses themselves.
    #[test]
    fn compile_falls_back_to_address_lookup() {
        let mut fallback_row = row(&[0], 5, &[1, 6], &[7]);
        fallback_row.executing_account_index = None;
        fallback_row.account_index_arguments = None;

        let (update, _) = assemble(&transaction(), vec![fallback_row], (None, None)).unwrap();
        let compiled = &update.transaction.message.instructions()[0];
        assert_eq!(compiled.program_id_index, 5);
        assert_eq!(compiled.accounts, vec![1, 6]);
    }

    /// Malformed inputs are rejected rather than panicking: a gap in the
    /// top-level sequence and a path deeper than Solana's stack limit.
    #[test]
    fn malformed_transactions_are_skipped_loudly() {
        let gap = vec![row(&[0], 5, &[1], &[1]), row(&[2], 5, &[1], &[1])];
        assert!(assemble(&transaction(), gap, (None, None))
            .unwrap_err()
            .contains("non-contiguous"));

        let deep = vec![
            row(&[0], 5, &[1], &[1]),
            row(&[0, 0, 0, 0, 0, 0], 5, &[1], &[1]),
        ];
        assert!(assemble(&transaction(), deep, (None, None))
            .unwrap_err()
            .contains("exceeds"));
    }
}
