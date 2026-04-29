//! Carbon Processor impls. Each match decodes its program's instruction and
//! submits a typed `ActivityCandidate` to `coordinator`. The coordinator
//! collapses candidates per (sig, wallet, family) and ships final events to
//! the writer — never one event per ix.

use crate::coordinator::{self, ActivityCandidate};
use crate::event::{
    extract_decimals, fmt_decimal, full_account_keys, iso8601_block_time,
    owner_of_token_account, safe_price_sol, token_balance_delta_raw, SurveillanceEventOut,
};
use crate::state::{self, WatchedWallet};
use crate::taxonomy::Activity;
use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult, instruction::InstructionProcessorInputType,
    metrics::MetricsCollection, processor::Processor,
};
use carbon_pump_swap_decoder::instructions::PumpSwapInstruction;
use carbon_pumpfun_decoder::instructions::{CpiEvent, PumpfunInstruction};
use carbon_system_program_decoder::instructions::SystemProgramInstruction;
use carbon_token_2022_decoder::instructions::Token2022Instruction;
use carbon_token_program_decoder::instructions::TokenProgramInstruction;
use solana_pubkey::Pubkey;
use std::{marker::PhantomData, sync::Arc};

// ─── PumpFun ──────────────────────────────────────────────────────────────

pub struct PumpfunWatch;

#[async_trait]
impl Processor for PumpfunWatch {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        if metadata.transaction_metadata.meta.status.is_err() {
            return Ok(());
        }
        let signer_str = metadata.transaction_metadata.fee_payer.to_string();
        let watched = match state::lookup(&signer_str) {
            Some(w) => w,
            None => return Ok(()),
        };

        // Only act on TradeEvent (post-execution, exact). Other variants either
        // pair with TradeEvent or aren't user trades worth surfacing.
        if let PumpfunInstruction::CpiEvent(boxed) = decoded.data {
            if let CpiEvent::TradeEvent(ev) = *boxed {
                let meta = &metadata.transaction_metadata.meta;
                let decimals =
                    extract_decimals(meta, &ev.user, &ev.mint).unwrap_or(6); // pump.fun default
                let signature = metadata.transaction_metadata.signature.to_string();
                let event = SurveillanceEventOut {
                    user_id: watched.user_id.clone(),
                    target_id: watched.target_id.clone(),
                    target_name: watched.target_name.clone(),
                    wallet_address: signer_str.clone(),
                    wallet_label: watched.wallet_label.clone(),
                    signature: signature.clone(),
                    event_type: if ev.is_buy { "swap_buy" } else { "swap_sell" },
                    token_address: Some(ev.mint.to_string()),
                    token_symbol: None,
                    sol_amount: ev.sol_amount as f64 / 1e9,
                    token_amount: fmt_decimal(ev.token_amount, decimals),
                    price_sol: safe_price_sol(ev.sol_amount, ev.token_amount, decimals),
                    program: "pumpfun",
                    counterparty: String::new(),
                    block_time: iso8601_block_time(metadata.transaction_metadata.block_time),
                    slot: metadata.transaction_metadata.slot,
                    raw_data: serde_json::json!({
                        "source": "frankfurt-carbon-surveillance",
                        "carbon_program": "pumpfun_decoder",
                        "ix_variant": "CpiEvent::TradeEvent",
                        "ix_name": ev.ix_name,
                        "is_mayhem_mode": ev.mayhem_mode,
                        "fee_basis_points": ev.fee_basis_points,
                        "fee_lamports": ev.fee,
                        "creator_fee_basis_points": ev.creator_fee_basis_points,
                        "creator_fee_lamports": ev.creator_fee,
                        "cashback_fee_basis_points": ev.cashback_fee_basis_points,
                        "cashback_lamports": ev.cashback,
                        "virtual_sol_reserves": ev.virtual_sol_reserves,
                        "virtual_token_reserves": ev.virtual_token_reserves,
                        "real_sol_reserves": ev.real_sol_reserves,
                        "real_token_reserves": ev.real_token_reserves,
                        "creator": ev.creator.to_string(),
                        "fee_recipient": ev.fee_recipient.to_string(),
                        "decimals": decimals,
                        "raw_token_amount": ev.token_amount,
                        "raw_sol_amount": ev.sol_amount,
                    }),
                };
                let activity = if ev.is_buy { Activity::SwapBuy } else { Activity::SwapSell };
                coordinator::submit(ActivityCandidate { activity, watched: watched.clone(), event }).await;
            }
        }
        Ok(())
    }
}

// ─── PumpSwap ─────────────────────────────────────────────────────────────

pub struct PumpSwapWatch;

#[async_trait]
impl Processor for PumpSwapWatch {
    type InputType = InstructionProcessorInputType<PumpSwapInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        if metadata.transaction_metadata.meta.status.is_err() {
            return Ok(());
        }
        let signer_str = metadata.transaction_metadata.fee_payer.to_string();
        let watched = match state::lookup(&signer_str) {
            Some(w) => w,
            None => return Ok(()),
        };

        // PumpSwap: extract base mint from accounts[3] (matches existing
        // decoder.rs convention). Buy/Sell variants tell us direction.
        let (event_type, ix_variant, activity) = match &decoded.data {
            PumpSwapInstruction::Buy(_) => ("swap_buy", "Buy", Activity::SwapBuy),
            PumpSwapInstruction::BuyExactQuoteIn(_) => {
                ("swap_buy", "BuyExactQuoteIn", Activity::SwapBuy)
            }
            PumpSwapInstruction::Sell(_) => ("swap_sell", "Sell", Activity::SwapSell),
            _ => return Ok(()),
        };
        let base_mint = match decoded.accounts.get(3) {
            Some(a) => a.pubkey,
            None => return Ok(()),
        };

        let meta = &metadata.transaction_metadata.meta;
        let signer_pk = metadata.transaction_metadata.fee_payer;
        let decimals = extract_decimals(meta, &signer_pk, &base_mint).unwrap_or(6);
        let token_delta_raw =
            token_balance_delta_raw(meta, &signer_pk, &base_mint).unwrap_or(0);
        let token_amount_raw = token_delta_raw.unsigned_abs() as u64;

        // SOL delta from native balance changes (account[0] of the wallet's
        // pre/post balances if available). PumpSwap doesn't expose sol_amount
        // in the ix args directly; rely on metadata.
        let sol_delta_lamports: i128 = {
            let static_keys =
                metadata.transaction_metadata.message.static_account_keys();
            if let Some(idx) = static_keys.iter().position(|k| k == &signer_pk) {
                let post = meta.post_balances.get(idx).copied().unwrap_or(0) as i128;
                let pre = meta.pre_balances.get(idx).copied().unwrap_or(0) as i128;
                post - pre
            } else {
                0
            }
        };
        let sol_amount_lamports = sol_delta_lamports.unsigned_abs() as u64;

        let signature = metadata.transaction_metadata.signature.to_string();
        let event = SurveillanceEventOut {
            user_id: watched.user_id.clone(),
            target_id: watched.target_id.clone(),
            target_name: watched.target_name.clone(),
            wallet_address: signer_str.clone(),
            wallet_label: watched.wallet_label.clone(),
            signature: signature.clone(),
            event_type,
            token_address: Some(base_mint.to_string()),
            token_symbol: None,
            sol_amount: sol_amount_lamports as f64 / 1e9,
            token_amount: fmt_decimal(token_amount_raw, decimals),
            price_sol: safe_price_sol(sol_amount_lamports, token_amount_raw, decimals),
            program: "pumpswap",
            counterparty: String::new(),
            block_time: iso8601_block_time(metadata.transaction_metadata.block_time),
            slot: metadata.transaction_metadata.slot,
            raw_data: serde_json::json!({
                "source": "frankfurt-carbon-surveillance",
                "carbon_program": "pump_swap_decoder",
                "ix_variant": ix_variant,
                "decimals": decimals,
                "raw_token_amount": token_amount_raw,
                "raw_sol_amount_lamports": sol_amount_lamports,
            }),
        };
        coordinator::submit(ActivityCandidate { activity, watched: watched.clone(), event }).await;
        Ok(())
    }
}

// ─── SOL transfers (SystemProgram::TransferSol / TransferSolWithSeed) ─────
// Source/destination of a SOL transfer ARE wallet pubkeys, so matching is
// direct: lookup against WATCH for either side. Emits one event per matching
// side (a self-transfer between the same watched wallet emits transfer_out
// once — the second side is the same wallet, suppressed by the dedup below).

pub struct TransferSolWatch;

#[async_trait]
impl Processor for TransferSolWatch {
    type InputType = InstructionProcessorInputType<SystemProgramInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        if metadata.transaction_metadata.meta.status.is_err() {
            return Ok(());
        }

        // Only outer (top-level) transfers. Inner-CPI'd SystemProgram transfers
        // are typically program internals (fees, rent, etc.), not user
        // transfers — frankfurt-node ignores them and so do we.
        if metadata.stack_height != 1 {
            return Ok(());
        }

        let (amount, source_pk, dest_pk, ix_variant) = match &decoded.data {
            SystemProgramInstruction::TransferSol(ix) => {
                let s = decoded.accounts.first().map(|a| a.pubkey);
                let d = decoded.accounts.get(1).map(|a| a.pubkey);
                match (s, d) {
                    (Some(s), Some(d)) => (ix.amount, s, d, "TransferSol"),
                    _ => return Ok(()),
                }
            }
            SystemProgramInstruction::TransferSolWithSeed(ix) => {
                // accounts: [source, base_account, destination, ...]
                let s = decoded.accounts.first().map(|a| a.pubkey);
                let d = decoded.accounts.get(2).map(|a| a.pubkey);
                match (s, d) {
                    (Some(s), Some(d)) => (ix.amount, s, d, "TransferSolWithSeed"),
                    _ => return Ok(()),
                }
            }
            _ => return Ok(()),
        };

        if amount == 0 {
            return Ok(());
        }

        let source_str = source_pk.to_string();
        let dest_str = dest_pk.to_string();
        let signer_str = metadata.transaction_metadata.fee_payer.to_string();
        let signature = metadata.transaction_metadata.signature.to_string();
        let block_time = iso8601_block_time(metadata.transaction_metadata.block_time);
        let slot = metadata.transaction_metadata.slot;

        let mut sides: Vec<(&'static str, String, String, WatchedWallet)> = Vec::new();
        if let Some(w) = state::lookup(&source_str) {
            sides.push(("transfer_out", source_str.clone(), dest_str.clone(), w));
        }
        if source_str != dest_str {
            if let Some(w) = state::lookup(&dest_str) {
                sides.push(("transfer_in", dest_str.clone(), source_str.clone(), w));
            }
        }

        for (event_type, wallet, counterparty, watched) in sides {
            let activity = if event_type == "transfer_in" {
                Activity::TransferIn
            } else {
                Activity::TransferOut
            };
            let event = SurveillanceEventOut {
                user_id: watched.user_id.clone(),
                target_id: watched.target_id.clone(),
                target_name: watched.target_name.clone(),
                wallet_address: wallet,
                wallet_label: watched.wallet_label.clone(),
                signature: signature.clone(),
                event_type,
                token_address: None,
                token_symbol: None,
                sol_amount: amount as f64 / 1e9,
                token_amount: 0.0,
                price_sol: 0.0,
                program: "system_program",
                counterparty,
                block_time: block_time.clone(),
                slot,
                raw_data: serde_json::json!({
                    "source": "frankfurt-carbon-surveillance",
                    "carbon_program": "system_program_decoder",
                    "ix_variant": ix_variant,
                    "raw_sol_amount_lamports": amount,
                    "fee_payer": signer_str,
                }),
            };
            coordinator::submit(ActivityCandidate { activity, watched: watched.clone(), event }).await;
        }
        Ok(())
    }
}

// ─── SPL transfers (TokenProgram + Token2022) ─────────────────────────────
// SPL `source`/`destination` are TOKEN ACCOUNTS, not wallets. The wallet
// (owner) of each token account is read from `post_token_balances` via
// `owner_of_token_account`. The `authority` slot in the ix's accounts is the
// sender's signer, but we don't rely on it for transfer_in detection — the
// recipient's wallet derives only from the destination token account's owner.
//
// Two concrete processors share `emit_spl_transfer_event` because TokenProgram
// and Token2022 expose distinct InstructionType enums — the per-variant
// dispatch differs but the event shape is identical.

#[allow(clippy::too_many_arguments)]
async fn emit_spl_transfer_event(
    program_label: &'static str,
    carbon_program: &'static str,
    ix_variant: &'static str,
    metadata: &Arc<carbon_core::transaction::TransactionMetadata>,
    source_token_account: Pubkey,
    dest_token_account: Pubkey,
    raw_amount: u64,
    explicit_decimals: Option<u8>,
    explicit_mint: Option<Pubkey>,
) {
    if raw_amount == 0 {
        return;
    }
    let meta = &metadata.meta;
    let static_keys = metadata.message.static_account_keys();
    let full_keys = full_account_keys(meta, static_keys);

    let src_info = owner_of_token_account(meta, &full_keys, &source_token_account);
    let dst_info = owner_of_token_account(meta, &full_keys, &dest_token_account);

    // Pick mint+decimals: explicit (TransferChecked) wins; else from balances.
    let (mint_str, decimals) = match (explicit_mint, explicit_decimals) {
        (Some(m), Some(d)) => (m.to_string(), d),
        _ => {
            if let Some((_, m, d)) = src_info.as_ref() {
                (m.clone(), *d)
            } else if let Some((_, m, d)) = dst_info.as_ref() {
                (m.clone(), *d)
            } else {
                return; // can't resolve mint — skip
            }
        }
    };

    let signer_str = metadata.fee_payer.to_string();
    let signature = metadata.signature.to_string();
    let block_time = iso8601_block_time(metadata.block_time);
    let slot = metadata.slot;

    let src_owner = src_info.as_ref().map(|(o, _, _)| o.clone());
    let dst_owner = dst_info.as_ref().map(|(o, _, _)| o.clone());

    let mut sides: Vec<(&'static str, String, String, WatchedWallet)> = Vec::new();
    if let Some(o) = src_owner.as_ref() {
        if let Some(w) = state::lookup(o) {
            let cp = dst_owner.clone().unwrap_or_default();
            sides.push(("transfer_out", o.clone(), cp, w));
        }
    }
    if let Some(o) = dst_owner.as_ref() {
        if Some(o) != src_owner.as_ref() {
            if let Some(w) = state::lookup(o) {
                let cp = src_owner.clone().unwrap_or_default();
                sides.push(("transfer_in", o.clone(), cp, w));
            }
        }
    }

    if sides.is_empty() {
        return;
    }

    let token_amount = fmt_decimal(raw_amount, decimals);
    for (event_type, wallet, counterparty, watched) in sides {
        let activity = if event_type == "transfer_in" {
            Activity::TransferIn
        } else {
            Activity::TransferOut
        };
        let event = SurveillanceEventOut {
            user_id: watched.user_id.clone(),
            target_id: watched.target_id.clone(),
            target_name: watched.target_name.clone(),
            wallet_address: wallet,
            wallet_label: watched.wallet_label.clone(),
            signature: signature.clone(),
            event_type,
            token_address: Some(mint_str.clone()),
            token_symbol: None,
            sol_amount: 0.0,
            token_amount,
            price_sol: 0.0,
            program: program_label,
            counterparty,
            block_time: block_time.clone(),
            slot,
            raw_data: serde_json::json!({
                "source": "frankfurt-carbon-surveillance",
                "carbon_program": carbon_program,
                "ix_variant": ix_variant,
                "decimals": decimals,
                "raw_token_amount": raw_amount,
                "source_token_account": source_token_account.to_string(),
                "dest_token_account": dest_token_account.to_string(),
                "fee_payer": signer_str,
            }),
        };
        coordinator::submit(ActivityCandidate { activity, watched: watched.clone(), event }).await;
    }
}

pub struct SplTransferWatch {
    pub program: &'static str,
}

impl SplTransferWatch {
    pub fn new(program: &'static str) -> Self {
        Self { program }
    }
}

#[async_trait]
impl Processor for SplTransferWatch {
    type InputType = InstructionProcessorInputType<TokenProgramInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        if metadata.transaction_metadata.meta.status.is_err() {
            return Ok(());
        }
        // Skip inner-CPI'd token transfers (swap legs, pool internals, etc.).
        // Only emit for top-level user-initiated transfers — matches
        // frankfurt-node's surveillance shape.
        if metadata.stack_height != 1 {
            return Ok(());
        }
        match &decoded.data {
            TokenProgramInstruction::Transfer(ix) => {
                // accounts: [source, destination, authority, ...]
                let src = match decoded.accounts.first() {
                    Some(a) => a.pubkey,
                    None => return Ok(()),
                };
                let dst = match decoded.accounts.get(1) {
                    Some(a) => a.pubkey,
                    None => return Ok(()),
                };
                emit_spl_transfer_event(
                    self.program,
                    "token_program_decoder",
                    "Transfer",
                    &metadata.transaction_metadata,
                    src,
                    dst,
                    ix.amount,
                    None,
                    None,
                )
                .await;
            }
            TokenProgramInstruction::TransferChecked(ix) => {
                // accounts: [source, mint, destination, authority, ...]
                let src = match decoded.accounts.first() {
                    Some(a) => a.pubkey,
                    None => return Ok(()),
                };
                let mint = decoded.accounts.get(1).map(|a| a.pubkey);
                let dst = match decoded.accounts.get(2) {
                    Some(a) => a.pubkey,
                    None => return Ok(()),
                };
                emit_spl_transfer_event(
                    self.program,
                    "token_program_decoder",
                    "TransferChecked",
                    &metadata.transaction_metadata,
                    src,
                    dst,
                    ix.amount,
                    Some(ix.decimals),
                    mint,
                )
                .await;
            }
            _ => {}
        }
        Ok(())
    }
}

pub struct Token2022TransferWatch {
    pub program: &'static str,
}

impl Token2022TransferWatch {
    pub fn new(program: &'static str) -> Self {
        Self { program }
    }
}

#[async_trait]
impl Processor for Token2022TransferWatch {
    type InputType = InstructionProcessorInputType<Token2022Instruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        if metadata.transaction_metadata.meta.status.is_err() {
            return Ok(());
        }
        if metadata.stack_height != 1 {
            return Ok(());
        }
        match &decoded.data {
            Token2022Instruction::Transfer(ix) => {
                let src = match decoded.accounts.first() {
                    Some(a) => a.pubkey,
                    None => return Ok(()),
                };
                let dst = match decoded.accounts.get(1) {
                    Some(a) => a.pubkey,
                    None => return Ok(()),
                };
                emit_spl_transfer_event(
                    self.program,
                    "token_2022_decoder",
                    "Transfer",
                    &metadata.transaction_metadata,
                    src,
                    dst,
                    ix.amount,
                    None,
                    None,
                )
                .await;
            }
            Token2022Instruction::TransferChecked(ix) => {
                let src = match decoded.accounts.first() {
                    Some(a) => a.pubkey,
                    None => return Ok(()),
                };
                let mint = decoded.accounts.get(1).map(|a| a.pubkey);
                let dst = match decoded.accounts.get(2) {
                    Some(a) => a.pubkey,
                    None => return Ok(()),
                };
                emit_spl_transfer_event(
                    self.program,
                    "token_2022_decoder",
                    "TransferChecked",
                    &metadata.transaction_metadata,
                    src,
                    dst,
                    ix.amount,
                    Some(ix.decimals),
                    mint,
                )
                .await;
            }
            _ => {}
        }
        Ok(())
    }
}

// ─── Generic aggregator/router/DEX surveillance processor ─────────────────
// Used for every other Carbon decoder. Falls back to post-pre balance deltas
// for amounts and accepts that we may emit zero-amount events when the wallet
// didn't actually move tokens (e.g., cross-program ix that only reads).
//
// EMIT_PROGRAMS controls which AggWatch instances actually emit. SKIP_PROGRAMS
// log-only. The list is set at registration time in main.rs, so we just take
// a flag + label here.

pub struct AggWatch<T> {
    pub program: &'static str,
    pub emit: bool,
    pub _marker: PhantomData<T>,
}

impl<T> AggWatch<T> {
    pub fn new(program: &'static str, emit: bool) -> Self {
        Self { program, emit, _marker: PhantomData }
    }
}

#[async_trait]
impl<T: Send + Sync + 'static> Processor for AggWatch<T> {
    type InputType = InstructionProcessorInputType<T>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, _decoded, _nested, raw) = data;
        if metadata.transaction_metadata.meta.status.is_err() {
            return Ok(());
        }

        // Anchor self-CPI event log: programs emit a CPI to themselves with
        // the instruction data being the event payload, prefixed with the
        // 8-byte sentinel `0xe445a52e51cb9a1d`. Carbon's decoders model these
        // as enum variants alongside real instructions, so without this gate
        // we emit one extra row per (sig, program) for any program that uses
        // Anchor events — e.g. raydium-launchpad's TradeEvent paired with
        // its own BuyExactIn outer ix. Skip them here so AggWatch emits only
        // for genuine instructions.
        const ANCHOR_EVENT_PREFIX: [u8; 8] =
            [228, 69, 165, 46, 81, 203, 154, 29];
        if raw.data.len() >= 8 && raw.data[..8] == ANCHOR_EVENT_PREFIX {
            return Ok(());
        }

        let signer_str = metadata.transaction_metadata.fee_payer.to_string();
        let watched = match state::lookup(&signer_str) {
            Some(w) => w,
            None => return Ok(()),
        };
        if !self.emit {
            return Ok(()); // SKIP_PROGRAMS: silent
        }

        // We don't know the variant or mint from a generic decoder match
        // without per-program logic. Submit a Fallback (ProgramActivity)
        // candidate — the coordinator will suppress it for any wallet that
        // also has a richer same-tx Swap/Mint/etc. candidate from a
        // dedicated processor.
        let signature = metadata.transaction_metadata.signature.to_string();
        let event = SurveillanceEventOut {
            user_id: watched.user_id.clone(),
            target_id: watched.target_id.clone(),
            target_name: watched.target_name.clone(),
            wallet_address: signer_str.clone(),
            wallet_label: watched.wallet_label.clone(),
            signature: signature.clone(),
            event_type: Activity::ProgramActivity.as_event_type(),
            token_address: None,
            token_symbol: None,
            sol_amount: 0.0,
            token_amount: 0.0,
            price_sol: 0.0,
            program: self.program,
            counterparty: String::new(),
            block_time: iso8601_block_time(metadata.transaction_metadata.block_time),
            slot: metadata.transaction_metadata.slot,
            raw_data: serde_json::json!({
                "source": "frankfurt-carbon-surveillance",
                "carbon_program": format!("{}_decoder", self.program),
                "note": "generic AggWatch — coordinator may suppress if richer candidate also fired",
            }),
        };
        coordinator::submit(ActivityCandidate {
            activity: Activity::ProgramActivity,
            watched: watched.clone(),
            event,
        })
        .await;
        Ok(())
    }
}
