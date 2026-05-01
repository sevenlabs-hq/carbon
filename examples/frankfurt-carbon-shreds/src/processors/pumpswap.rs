//! PumpSwap (post-graduation) direct-trade processor.
//!
//! Same shape as PumpFun: decode the typed top-level Buy/Sell ix on shreds.
//! PumpSwap is the AMM pump.fun tokens migrate to after their bonding curve
//! completes — once a token graduates, all subsequent trades route through
//! the PumpSwap program. Both phases of the lifecycle (pre-grad pumpfun +
//! post-grad pumpswap) need to be caught for the default
//! `launchpad_filter=["pumpfun"]` user expectation to hold.

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
use carbon_pump_swap_decoder::instructions::{
    buy::Buy, buy_exact_quote_in::BuyExactQuoteIn, sell::Sell, PumpSwapInstruction,
};
use std::sync::{atomic::Ordering, Arc};

const FAMILY: &str = "pumpswap";
const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

pub struct PumpSwapShredWatch;

#[async_trait]
impl Processor for PumpSwapShredWatch {
    type InputType = InstructionProcessorInputType<PumpSwapInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        metrics::TX_SEEN.fetch_add(1, Ordering::Relaxed);

        // Account layout comes from Carbon's typed `ArrangeAccounts` impls.
        // Quote-leg is wSOL for pumpfun-graduated pools, so quote_amount is
        // the SOL leg either way. Field names verified against
        //   decoders/pump-swap-decoder/src/instructions/{buy,buy_exact_quote_in,sell}.rs
        // Buy             -> { base_amount_out, max_quote_amount_in }
        // BuyExactQuoteIn -> { spendable_quote_in, min_base_amount_out, track_volume }
        // Sell            -> { base_amount_in, min_quote_amount_out }
        // No SellExactBaseOut/SellExactQuoteOut variant exists in the current
        // Codama-generated enum.
        let extracted = match &decoded.data {
            PumpSwapInstruction::Buy(args) => {
                Buy::arrange_accounts(&decoded.accounts).map(|a| Trade {
                    signer: a.user.to_string(),
                    mint: a.base_mint.to_string(),
                    pool: Some(a.pool.to_string()),
                    is_buy: true,
                    sol_lamports: args.max_quote_amount_in,
                })
            }
            PumpSwapInstruction::BuyExactQuoteIn(args) => {
                BuyExactQuoteIn::arrange_accounts(&decoded.accounts).map(|a| Trade {
                    signer: a.user.to_string(),
                    mint: a.base_mint.to_string(),
                    pool: Some(a.pool.to_string()),
                    is_buy: true,
                    sol_lamports: args.spendable_quote_in,
                })
            }
            PumpSwapInstruction::Sell(args) => {
                Sell::arrange_accounts(&decoded.accounts).map(|a| Trade {
                    signer: a.user.to_string(),
                    mint: a.base_mint.to_string(),
                    pool: Some(a.pool.to_string()),
                    is_buy: false,
                    sol_lamports: args.min_quote_amount_out,
                })
            }
            // Other variants — pool create, admin, migration — are not user
            // trades to surface. Counted as decoded for visibility.
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
        let observed_ts_ms = now_ms();

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
            pool: trade.pool,
            matched_users: watchers,
        });
        Ok(())
    }
}

struct Trade {
    signer: String,
    mint: String,
    pool: Option<String>,
    is_buy: bool,
    sol_lamports: u64,
}

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
