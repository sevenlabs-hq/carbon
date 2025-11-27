use {
    async_trait::async_trait,
    carbon_core::{
        datasource::{
            AccountDeletion, AccountUpdate, Datasource, DatasourceId, TransactionUpdate, Update,
            UpdateType,
        },
        error::CarbonResult,
        metrics::MetricsCollection,
    },
    futures::StreamExt,
    helius::{
        types::{Cluster, RpcTransactionsConfig},
        websocket::EnhancedWebsocket,
        Helius,
    },
    solana_account::Account,
    solana_clock::Clock,
    solana_message::{compiled_instruction::CompiledInstruction, v0::LoadedAddresses},
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    solana_transaction_context::TransactionReturnData,
    solana_transaction_status::{
        option_serializer::OptionSerializer, InnerInstruction, InnerInstructions, Reward,
        TransactionStatusMeta, TransactionTokenBalance, UiInstruction, UiLoadedAddresses,
    },
    std::{
        collections::HashSet,
        str::FromStr,
        sync::Arc,
        time::{Duration, Instant},
    },
    tokio::sync::{mpsc::Sender, RwLock},
    tokio_util::sync::CancellationToken,
};

const DEVNET_WS_URL: &str = "wss://atlas-devnet.helius-rpc.com/";
const MAINNET_WS_URL: &str = "wss://atlas-mainnet.helius-rpc.com/";
const MAX_MISSED_BLOCKS: u64 = 10;
const MAX_RECONNECTION_ATTEMPTS: u32 = 30;
const RECONNECTION_DELAY_MS: u64 = 3000;

#[derive(Debug, Clone)]
pub struct Filters {
    pub accounts: Vec<Pubkey>,
    pub transactions: Option<RpcTransactionsConfig>,
}

impl Filters {
    pub fn new(
        accounts: Vec<Pubkey>,
        transactions: Option<RpcTransactionsConfig>,
    ) -> CarbonResult<Self> {
        if accounts.is_empty() && transactions.is_none() {
            return CarbonResult::Err(carbon_core::error::Error::Custom("Error creating Filters for the Helius WebSocket: accounts and transactions can't be both empty".to_string()));
        };

        Ok(Filters {
            accounts,
            transactions,
        })
    }
}

pub struct HeliusWebsocket {
    pub api_key: String,
    pub filters: Filters,
    pub account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
    pub cluster: Cluster,
}

impl HeliusWebsocket {
    pub const fn new(
        api_key: String,
        filters: Filters,
        account_deletions_tracked: Arc<RwLock<HashSet<Pubkey>>>,
        cluster: Cluster,
    ) -> Self {
        Self {
            api_key,
            filters,
            account_deletions_tracked,
            cluster,
        }
    }

    const fn get_ws_url(cluster: &Cluster) -> &'static str {
        match cluster {
            Cluster::MainnetBeta => MAINNET_WS_URL,
            _ => DEVNET_WS_URL,
        }
    }
}
#[async_trait]
impl Datasource for HeliusWebsocket {
    async fn consume(
        &self,
        id: DatasourceId,
        sender: Sender<(Update, DatasourceId)>,
        cancellation_token: CancellationToken,
        metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        if self.filters.accounts.is_empty() && self.filters.transactions.is_none() {
            return CarbonResult::Err(carbon_core::error::Error::Custom("Error creating Filters for the Helius WebSocket: accounts and transactions can't be both empty".to_string()));
        }

        let mut reconnection_attempts = 0;

        let id = id.clone();

        loop {
            if cancellation_token.is_cancelled() {
                log::info!("Cancellation requested, stopping reconnection attempts");
                break;
            }

            let mut helius = match Helius::new(&self.api_key, self.cluster.clone()) {
                Ok(client) => client,
                Err(err) => {
                    log::error!("Failed to create Helius client: {err}");
                    reconnection_attempts += 1;
                    if reconnection_attempts >= MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to create Helius client after {MAX_RECONNECTION_ATTEMPTS} attempts: {err}"
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
                    continue;
                }
            };

            let ws_url = format!(
                "{}/?api-key={}",
                Self::get_ws_url(&self.cluster),
                self.api_key
            );

            let ws = match EnhancedWebsocket::new(&ws_url, None, None).await {
                Ok(ws) => ws,
                Err(err) => {
                    log::error!("Failed to create Enhanced Helius Websocket: {err}");
                    reconnection_attempts += 1;
                    if reconnection_attempts >= MAX_RECONNECTION_ATTEMPTS {
                        return Err(carbon_core::error::Error::Custom(format!(
                            "Failed to create Enhanced Helius Websocket after {MAX_RECONNECTION_ATTEMPTS} attempts: {err}"
                        )));
                    }
                    tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
                    continue;
                }
            };

            helius.ws_client = Some(Arc::new(ws));

            let account_deletions_tracked = Arc::clone(&self.account_deletions_tracked);
            let filters = self.filters.clone();
            let sender = sender.clone();
            let helius = Arc::new(helius);
            let metrics = Arc::clone(&metrics);

            let iteration_cancellation = CancellationToken::new();
            let iteration_cancellation_clone = iteration_cancellation.clone();

            let main_cancellation = cancellation_token.clone();
            let id_for_loop = id.clone();

            let handle = tokio::spawn(async move {
                let mut handles = vec![];

                // Clock subscription
                let cancellation_token_clock = main_cancellation.clone();
                let iteration_cancellation_clock = iteration_cancellation.clone();
                let helius_clone = Arc::clone(&helius);
                let metrics_clone = Arc::clone(&metrics);

                let handle = tokio::spawn(async move {
                    let ws = match helius_clone.ws() {
                        Some(ws) => ws,
                        None => {
                            log::error!("Helius Websocket not available for Clock subscription");
                            iteration_cancellation_clock.cancel();
                            return;
                        }
                    };

                    let (mut stream, _unsub) = match ws
                        .account_subscribe(&solana_program::sysvar::clock::ID, None)
                        .await
                    {
                        Ok(subscription) => subscription,
                        Err(err) => {
                            log::error!("Failed to subscribe to Clock sysvar: {err:?}");
                            iteration_cancellation_clock.cancel();
                            return;
                        }
                    };

                    let mut last_slot = 0u64;
                    let mut last_clock_update = Instant::now();

                    loop {
                        tokio::select! {
                            _ = cancellation_token_clock.cancelled() => {
                                return;
                            }
                            _ = iteration_cancellation_clock.cancelled() => {
                                return;
                            }
                            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                                if last_clock_update.elapsed() > Duration::from_secs(5) {
                                    log::warn!("No Clock updates received for 5 seconds, triggering reconnection");
                                    iteration_cancellation_clock.cancel();
                                    return;
                                }
                            }
                            event_result = stream.next() => {
                                match event_result {
                                    Some(clock_event) => {
                                        last_clock_update = Instant::now();
                                        if let Some(clock_data) = clock_event.value.decode::<Account>() {
                                            if let Ok(clock) = bincode::deserialize::<Clock>(&clock_data.data) {
                                                let current_slot = clock.slot;

                                                if last_slot > 0 && current_slot > last_slot + MAX_MISSED_BLOCKS {
                                                    log::warn!(
                                                        "Detected large slot gap: last_slot={}, current_slot={}, gap={}",
                                                        last_slot, current_slot, current_slot - last_slot
                                                    );
                                                    iteration_cancellation_clock.cancel();
                                                    return;
                                                }
                                                last_slot = current_slot;

                                                metrics_clone
                                                    .record_histogram(
                                                        "helius_atlas_ws_clock_process_time_nanoseconds",
                                                        last_clock_update.elapsed().as_nanos() as f64
                                                    )
                                                    .await
                                                    .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));
                                            }
                                        }
                                    }
                                    None => {
                                        log::warn!("Clock sysvar stream closed, triggering reconnection");
                                        iteration_cancellation_clock.cancel();
                                        return;
                                    }
                                }
                            }
                        }
                    }
                });

                handles.push(handle);

                // Account subscriptions
                if !filters.accounts.is_empty() {
                    for account in filters.accounts {
                        let cancellation_token_acc = main_cancellation.clone();
                        let iteration_cancellation_acc = iteration_cancellation.clone();
                        let sender_clone = sender.clone();
                        let helius_clone = Arc::clone(&helius);
                        let account_deletions_tracked = Arc::clone(&account_deletions_tracked);
                        let metrics = metrics.clone();
                        let id_for_account = id_for_loop.clone();

                        let handle = tokio::spawn(async move {
                            let ws = match helius_clone.ws() {
                                Some(ws) => ws,
                                None => {
                                    log::error!("Helius Websocket not available");
                                    return;
                                }
                            };

                            let (mut stream, _unsub) =
                                match ws.account_subscribe(&account, None).await {
                                    Ok(subscription) => subscription,
                                    Err(err) => {
                                        log::error!(
                                            "Failed to subscribe to account {account}: {err:?}"
                                        );
                                        return;
                                    }
                                };

                            loop {
                                tokio::select! {
                                    _ = cancellation_token_acc.cancelled() => {
                                        log::info!("Main cancellation requested for account subscription");
                                        return;
                                    }
                                    _ = iteration_cancellation_acc.cancelled() => {
                                        log::info!("Iteration cancelled for account subscription");
                                        return;
                                    }
                                    event_result = stream.next() => {
                                        match event_result {
                                            Some(acc_event) => {
                                                let start_time = std::time::Instant::now();
                                                let decoded_account: Account = match acc_event.value.decode() {
                                                    Some(account_data) => account_data,
                                                    None => {
                                                        log::error!("Error decoding Helius WS Account event");
                                                        continue;
                                                    }
                                                };

                                                if decoded_account.lamports == 0 && decoded_account.data.is_empty() && decoded_account.owner == solana_system_interface::program::ID {
                                                    let accounts_tracked =
                                                        account_deletions_tracked.read().await;
                                                    if !accounts_tracked.is_empty() && accounts_tracked.contains(&account) {
                                                        let account_deletion = AccountDeletion {
                                                            pubkey: account,
                                                            slot: acc_event.context.slot,
                                                            transaction_signature: None,
                                                        };

                                                        metrics.record_histogram("helius_atlas_ws_account_deletion_process_time_nanoseconds", start_time.elapsed().as_nanos() as f64).await.unwrap_or_else(|value| log::error!("Error recording metric: {value}"));

                                                        metrics.increment_counter("helius_atlas_ws_account_deletions_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {value}"));


                                                        if let Err(err) = sender_clone.try_send((
                                                            Update::AccountDeletion(account_deletion),
                                                            id_for_account.clone(),
                                                        )) {
                                                            log::error!("Error sending account update: {err:?}");
                                                            break;
                                                        }
                                                    }
                                                } else {
                                                    let update = Update::Account(AccountUpdate {
                                                        pubkey: account,
                                                        account: decoded_account,
                                                        slot: acc_event.context.slot,
                                                        transaction_signature: None,
                                                    });

                                                    metrics.record_histogram("helius_atlas_ws_account_process_time_nanoseconds", start_time.elapsed().as_nanos() as f64).await.unwrap_or_else(|value| log::error!("Error recording metric: {value}"));

                                                    metrics.increment_counter("helius_atlas_ws_account_updates_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {value}"));


                                                    if let Err(err) = sender_clone.try_send((
                                                        update,
                                                        id_for_account.clone(),
                                                    )) {
                                                        log::error!("Error sending account update: {err:?}");
                                                        break;
                                                    }
                                                }
                                            },
                                            None => {
                                                log::info!("Helius WS Accounts stream has been closed");
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        });

                        handles.push(handle);
                    }
                }

                // Transaction subscription
                if let Some(config) = filters.transactions {
                    let cancellation_token_tx = main_cancellation.clone();
                    let iteration_cancellation_tx = iteration_cancellation.clone();
                    let sender_clone = sender.clone();
                    let helius_clone = Arc::clone(&helius);
                    let id_for_transaction = id_for_loop.clone();

                    let handle = tokio::spawn(async move {
                        let ws = match helius_clone.ws() {
                            Some(ws) => ws,
                            None => {
                                log::error!("Helius Websocket not available");
                                return;
                            }
                        };

                        let (mut stream, _unsub) =
                            match ws.transaction_subscribe(config.clone()).await {
                                Ok(subscription) => subscription,
                                Err(err) => {
                                    log::error!("Failed to subscribe to transactions: {err:?}");
                                    return;
                                }
                            };

                        loop {
                            tokio::select! {
                                _ = cancellation_token_tx.cancelled() => {
                                    log::info!("Main cancellation requested for transaction subscription");
                                    return;
                                }
                                _ = iteration_cancellation_tx.cancelled() => {
                                    log::info!("Iteration cancelled for transaction subscription");
                                    return;
                                }
                                event_result = stream.next() => {
                                    match event_result {
                                        Some(tx_event) => {
                                            let start_time = std::time::Instant::now();
                                            let encoded_transaction_with_status_meta = tx_event.transaction;
                                            let signature_str = tx_event.signature;
                                            let Ok(signature) = Signature::from_str(&signature_str) else {
                                                log::error!("Error getting Signature from string");
                                                continue;
                                            };

                                            let meta_original = if let Some(meta) = encoded_transaction_with_status_meta.clone().meta {
                                                meta
                                            } else {
                                                log::warn!("Meta is malformed for transaction: {signature_str:?}");
                                                continue;
                                            };

                                            if meta_original.status.is_err() {
                                                continue;
                                            }

                                            let Some(decoded_transaction) = encoded_transaction_with_status_meta.transaction.decode() else {
                                                log::error!("Failed to decode transaction: {encoded_transaction_with_status_meta:?}");
                                                continue;
                                            };

                                            let meta_needed = TransactionStatusMeta {
                                                status: meta_original.status.map_err(Into::into),
                                                fee: meta_original.fee,
                                                pre_balances: meta_original.pre_balances,
                                                post_balances: meta_original.post_balances,
                                                inner_instructions: Some(
                                                    meta_original
                                                        .inner_instructions
                                                        .unwrap_or_else(std::vec::Vec::new)
                                                        .iter()
                                                        .map(|inner_instruction_group| InnerInstructions {
                                                            index: inner_instruction_group.index,
                                                            instructions: inner_instruction_group
                                                                .instructions
                                                                .iter()
                                                                .map(|ui_instruction| match ui_instruction {
                                                                    UiInstruction::Compiled(compiled_ui_instruction) => {
                                                                        let decoded_data = bs58::decode(
                                                                            compiled_ui_instruction.data.clone(),
                                                                        )
                                                                        .into_vec()
                                                                        .unwrap_or_else(|_| vec![]);
                                                                        InnerInstruction {
                                                                            instruction: CompiledInstruction {
                                                                                program_id_index: compiled_ui_instruction
                                                                                    .program_id_index,
                                                                                accounts: compiled_ui_instruction
                                                                                    .accounts
                                                                                    .clone(),
                                                                                data: decoded_data,
                                                                            },
                                                                            stack_height: compiled_ui_instruction
                                                                                .stack_height,
                                                                        }
                                                                    }
                                                                    _ => {
                                                                        log::error!(
                                                                            "Unsupported instruction type encountered"
                                                                        );
                                                                        InnerInstruction {
                                                                            instruction: CompiledInstruction {
                                                                                program_id_index: 0,
                                                                                accounts: vec![],
                                                                                data: vec![],
                                                                            },
                                                                            stack_height: None,
                                                                        }
                                                                    }
                                                                })
                                                                .collect::<Vec<InnerInstruction>>(),
                                                        })
                                                        .collect::<Vec<InnerInstructions>>(),
                                                ),
                                                log_messages: Some(meta_original.log_messages.unwrap_or_else(std::vec::Vec::new)),
                                                pre_token_balances: Some(
                                                    meta_original
                                                        .pre_token_balances
                                                        .unwrap_or_else(std::vec::Vec::new)
                                                        .iter()
                                                        .filter_map(|transaction_token_balance| {
                                                            if let (
                                                                OptionSerializer::Some(owner),
                                                                OptionSerializer::Some(program_id),
                                                            ) = (
                                                                transaction_token_balance.owner.as_ref(),
                                                                transaction_token_balance.program_id.as_ref(),
                                                            ) {
                                                                Some(TransactionTokenBalance {
                                                                    account_index: transaction_token_balance.account_index,
                                                                    mint: transaction_token_balance.mint.clone(),
                                                                    ui_token_amount: transaction_token_balance
                                                                        .ui_token_amount
                                                                        .clone(),
                                                                    owner: owner.to_string(),
                                                                    program_id: program_id.to_string(),
                                                                })
                                                            } else {
                                                                None
                                                            }
                                                        })
                                                        .collect::<Vec<TransactionTokenBalance>>(),
                                                ),
                                                post_token_balances: Some(
                                                    meta_original
                                                        .post_token_balances
                                                        .unwrap_or_else(std::vec::Vec::new)
                                                        .iter()
                                                        .filter_map(|transaction_token_balance| {
                                                            if let (
                                                                OptionSerializer::Some(owner),
                                                                OptionSerializer::Some(program_id),
                                                            ) = (
                                                                transaction_token_balance.owner.as_ref(),
                                                                transaction_token_balance.program_id.as_ref(),
                                                            ) {
                                                                Some(TransactionTokenBalance {
                                                                    account_index: transaction_token_balance.account_index,
                                                                    mint: transaction_token_balance.mint.clone(),
                                                                    ui_token_amount: transaction_token_balance
                                                                        .ui_token_amount
                                                                        .clone(),
                                                                    owner: owner.to_string(),
                                                                    program_id: program_id.to_string(),
                                                                })
                                                            } else {
                                                                None
                                                            }
                                                        })
                                                        .collect::<Vec<TransactionTokenBalance>>(),
                                                ),
                                                rewards: Some(
                                                    meta_original
                                                        .rewards
                                                        .unwrap_or_else(std::vec::Vec::new)
                                                        .iter()
                                                        .map(|rewards| Reward {
                                                            pubkey: rewards.pubkey.clone(),
                                                            lamports: rewards.lamports,
                                                            post_balance: rewards.post_balance,
                                                            reward_type: rewards.reward_type,
                                                            commission: rewards.commission,
                                                        })
                                                        .collect::<Vec<Reward>>(),
                                                ),
                                                loaded_addresses: {
                                                    let loaded = meta_original.loaded_addresses.unwrap_or_else(|| {
                                                        UiLoadedAddresses {
                                                            writable: vec![],
                                                            readonly: vec![],
                                                        }
                                                    });
                                                    LoadedAddresses {
                                                        writable: loaded
                                                            .writable
                                                            .iter()
                                                            .map(|w| Pubkey::from_str(w).unwrap_or_default())
                                                            .collect::<Vec<Pubkey>>(),
                                                        readonly: loaded
                                                            .readonly
                                                            .iter()
                                                            .map(|r| Pubkey::from_str(r).unwrap_or_default())
                                                            .collect::<Vec<Pubkey>>(),
                                                    }
                                                },
                                                return_data: meta_original.return_data.map(|return_data| {
                                                    TransactionReturnData {
                                                        program_id: return_data.program_id.parse().unwrap_or_default(),
                                                        data: return_data.data.0.as_bytes().to_vec(),
                                                    }
                                                }),
                                                compute_units_consumed: meta_original
                                                    .compute_units_consumed
                                                    .map(|compute_unit_consumed| compute_unit_consumed)
                                                    .or(None),
                                                cost_units: meta_original.cost_units.into(),
                                            };

                                            let update = Update::Transaction(Box::new(TransactionUpdate {
                                                signature,
                                                transaction: decoded_transaction.clone(),
                                                meta: meta_needed,
                                                is_vote: config.filter.vote.is_some_and(|is_vote| is_vote),
                                                slot: tx_event.slot,
                                                block_time: None,
                                                block_hash: None,
                                            }));

                                            metrics
                                                    .record_histogram(
                                                        "helius_atlas_ws_transaction_process_time_nanoseconds",
                                                        start_time.elapsed().as_nanos() as f64
                                                    )
                                                    .await
                                                    .unwrap_or_else(|value| log::error!("Error recording metric: {value}"));

                                            metrics.increment_counter("helius_atlas_ws_transaction_updates_received", 1).await.unwrap_or_else(|value| log::error!("Error recording metric: {value}"));


                                            if let Err(err) = sender_clone.try_send((
                                                update,
                                                id_for_transaction.clone(),
                                            )) {
                                                log::error!("Error sending transaction update: {err:?}");
                                                break;
                                            }
                                        },
                                        None => {
                                            log::info!("Helius WS Accounts stream has been closed");
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    });

                    handles.push(handle);
                }

                for handle in handles {
                    if let Err(e) = handle.await {
                        log::error!("Helius WS Task failed: {e:?}");
                    }
                }

                iteration_cancellation.cancel();
            });

            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    iteration_cancellation_clone.cancel();
                    break;
                }
                _ = iteration_cancellation_clone.cancelled() => {

                }
                result = handle => {
                    if let Err(e) = result {
                        log::error!("Main task failed: {e:?}");
                    }
                }
            }

            reconnection_attempts = 0;
            tokio::time::sleep(Duration::from_millis(RECONNECTION_DELAY_MS)).await;
        }

        Ok(())
    }

    fn update_types(&self) -> Vec<UpdateType> {
        vec![
            UpdateType::Transaction,
            UpdateType::AccountUpdate,
            UpdateType::AccountDeletion,
        ]
    }
}
