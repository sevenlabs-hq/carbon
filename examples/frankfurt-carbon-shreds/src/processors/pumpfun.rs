//! PumpFun direct-trade processor for the shreds path.
//!
//! Critical difference from the surveillance processor: shreds carry
//! transaction *bodies* but no execution metadata, so `CpiEvent::TradeEvent`
//! (which surveillance relies on) IS NOT VISIBLE on shreds — those events
//! live in inner-instruction logs that don't exist pre-execution. Here we
//! decode the **direct** Buy / Sell variants instead. That's what Codama
//! generates for the typed top-level ix path, and it matches frankfurt-
//! rust's hand-rolled decoder.rs in coverage shape — but with all 4 trade
//! variants instead of 1, plus correct accounts arrangement instead of
//! numeric `accounts[2]` indexing.
//!
//! Routed trades (Jupiter → pumpfun, DFlow → pumpfun) appear here only as
//! inner CPIs — invisible on shreds. The aggregator-side processor
//! (Stage 3 work) will extract those from Jupiter's typed `Route` args.

use crate::blocklist;
use crate::coordinator;
use crate::event::ObservedTrigger;
use crate::metrics;
use crate::state;
use async_trait::async_trait;
use carbon_core::{
    deserialize::ArrangeAccounts, error::CarbonResult, instruction::InstructionProcessorInputType,
    metrics::MetricsCollection, processor::Processor,
};
use carbon_pumpfun_decoder::instructions::{
    buy::Buy, buy_exact_sol_in::BuyExactSolIn, sell::Sell, PumpfunInstruction,
};
use std::sync::{atomic::Ordering, Arc};

const FAMILY: &str = "pumpfun";
const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

pub struct PumpfunShredWatch;

#[async_trait]
impl Processor for PumpfunShredWatch {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        metrics::TX_SEEN.fetch_add(1, Ordering::Relaxed);

        // Resolve (signer, mint, is_buy, sol_lamports) by variant. Account
        // layout comes from Carbon's typed `ArrangeAccounts` impls — never
        // numeric indexing — so any future Codama regen that reshuffles
        // account order is caught at compile time, not silently mis-decoded.
        // Arg field names verified against
        //   decoders/pumpfun-decoder/src/instructions/{buy,buy_exact_sol_in,sell}.rs
        // Buy            -> { amount, max_sol_cost, track_volume }      (max_sol_cost = SOL cap)
        // BuyExactSolIn  -> { spendable_sol_in, min_tokens_out, ... }   (spendable_sol_in = SOL in)
        // Sell           -> { amount, min_sol_output }                  (min_sol_output = SOL floor)
        // No SellExactSolOut variant exists in the current Codama-generated enum.
        let extracted = match &decoded.data {
            PumpfunInstruction::Buy(_args) => {
                Buy::arrange_accounts(&decoded.accounts).map(|a| Extracted {
                    signer: a.user.to_string(),
                    mint: a.mint.to_string(),
                    is_buy: true,
                    sol_lamports: _args.max_sol_cost,
                })
            }
            PumpfunInstruction::BuyExactSolIn(args) => {
                BuyExactSolIn::arrange_accounts(&decoded.accounts).map(|a| Extracted {
                    signer: a.user.to_string(),
                    mint: a.mint.to_string(),
                    is_buy: true,
                    sol_lamports: args.spendable_sol_in,
                })
            }
            PumpfunInstruction::Sell(args) => {
                Sell::arrange_accounts(&decoded.accounts).map(|a| Extracted {
                    signer: a.user.to_string(),
                    mint: a.mint.to_string(),
                    is_buy: false,
                    sol_lamports: args.min_sol_output,
                })
            }
            // Everything else (Create, admin/fee/migration ix variants) is
            // not a trade signal. Counted as decoded but not emitted.
            _ => {
                metrics::inc_decoded(FAMILY);
                return Ok(());
            }
        };

        metrics::inc_decoded(FAMILY);

        let Some(trade) = extracted else {
            metrics::inc_extraction_failed(FAMILY);
            return Ok(());
        };

        // Defense-in-depth: never trigger on a router/tip wallet. Watch
        // list ingestion blocks these at API level (PR #171), but we
        // re-check here so any drift can't surface a bad trigger.
        if blocklist::is_blocked(&trade.signer) {
            metrics::SKIPPED_BLOCKLISTED.fetch_add(1, Ordering::Relaxed);
            return Ok(());
        }

        let watchers = state::watchers_for(&trade.signer);
        if watchers.is_empty() {
            metrics::SKIPPED_NOT_WATCHED.fetch_add(1, Ordering::Relaxed);
            return Ok(());
        }

        let signature = metadata.transaction_metadata.signature.to_string();
        let slot = metadata.transaction_metadata.slot;
        let observed_ts_ms = chrono_now_ms();

        coordinator::submit(ObservedTrigger {
            signature,
            slot,
            observed_ts_ms,
            signer: trade.signer,
            mint: trade.mint,
            is_buy: trade.is_buy,
            sol_amount: Some(trade.sol_lamports as f64 / LAMPORTS_PER_SOL),
            decoder_family: FAMILY,
            source_route: None,
            pool: None, // PumpFun bonding curve, not a pool addr
            matched_users: watchers,
        });
        Ok(())
    }
}

struct Extracted {
    signer: String,
    mint: String,
    is_buy: bool,
    sol_lamports: u64,
}

fn chrono_now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
