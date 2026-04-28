//! Carbon Processor impls. Each match runs only when the tx's signer is in the
//! WATCH map. Output goes through `crate::writer::send_event` (bounded mpsc).

use crate::event::{
    extract_decimals, fmt_decimal, iso8601_block_time, safe_price_sol,
    token_balance_delta_raw, SurveillanceEventOut,
};
use crate::state;
use crate::writer;
use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult, instruction::InstructionProcessorInputType,
    metrics::MetricsCollection, processor::Processor,
};
use carbon_pump_swap_decoder::instructions::PumpSwapInstruction;
use carbon_pumpfun_decoder::instructions::{CpiEvent, PumpfunInstruction};
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
                writer::send_event(event).await;
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
        let signer_str = metadata.transaction_metadata.fee_payer.to_string();
        let watched = match state::lookup(&signer_str) {
            Some(w) => w,
            None => return Ok(()),
        };

        // PumpSwap: extract base mint from accounts[3] (matches existing
        // decoder.rs convention). Buy/Sell variants tell us direction.
        let (event_type, ix_variant) = match &decoded.data {
            PumpSwapInstruction::Buy(_) => ("swap_buy", "Buy"),
            PumpSwapInstruction::BuyExactQuoteIn(_) => ("swap_buy", "BuyExactQuoteIn"),
            PumpSwapInstruction::Sell(_) => ("swap_sell", "Sell"),
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
        writer::send_event(event).await;
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
        let (metadata, _decoded, _nested, _raw) = data;
        let signer_str = metadata.transaction_metadata.fee_payer.to_string();
        let watched = match state::lookup(&signer_str) {
            Some(w) => w,
            None => return Ok(()),
        };
        if !self.emit {
            return Ok(()); // SKIP_PROGRAMS: silent
        }

        // We don't know the mint from a generic ix without per-program logic;
        // emit a coarse event with what we have. The frontend can still show
        // "wallet X used program Y" rows even without exact amounts.
        let signature = metadata.transaction_metadata.signature.to_string();
        let event = SurveillanceEventOut {
            user_id: watched.user_id.clone(),
            target_id: watched.target_id.clone(),
            target_name: watched.target_name.clone(),
            wallet_address: signer_str.clone(),
            wallet_label: watched.wallet_label.clone(),
            signature: signature.clone(),
            event_type: "swap_buy", // conservatively label as buy; frontend dedups by sig
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
                "note": "generic AggWatch — amounts/mint require per-program extraction",
            }),
        };
        writer::send_event(event).await;
        Ok(())
    }
}
