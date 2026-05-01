//! Per-program processors. Each one matches Carbon's typed instruction enum
//! for its program, identifies the trader, and submits an `ObservedTrigger`
//! to the coordinator. The coordinator collapses per-(sig, signer, family)
//! and publishes through the writer.
//!
//! Stage 1 ships full extraction for PumpFun and PumpSwap (covering the
//! default `launchpad_filter=["pumpfun"]` case for users) plus a generic
//! `AggWatch` processor used for every other registered decoder. AggWatch
//! increments the per-program metric counter so we can quantify volume on
//! each aggregator/AMM during the soak — without emitting triggers yet.
//! Adding extraction for an aggregator (Jupiter Route, DFlow Swap, etc.)
//! is a per-program PR in Stage 3.

pub mod agg_watch;
pub mod pumpfun;
pub mod pumpswap;

pub use agg_watch::AggWatch;
pub use pumpfun::PumpfunShredWatch;
pub use pumpswap::PumpSwapShredWatch;
