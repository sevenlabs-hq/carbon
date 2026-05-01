//! Generic metric-only processor used for every decoder in Stage 1 except
//! PumpFun and PumpSwap.
//!
//! Increments `decoded_per_program{family}` for each ix Carbon hands us.
//! Does NOT submit `ObservedTrigger`s — extraction logic for each
//! aggregator/AMM (Jupiter Route, DFlow Swap, OKX, Raydium AMM/CLMM/CPMM/
//! Launchpad/StableSwap, Meteora variants, Orca Whirlpool, …) lands per
//! program in Stage 3.
//!
//! The point of registering decoders behind AggWatch even before extraction
//! is wired is so the soak's `/stats` output tells us **how much of our
//! tracked-wallet traffic each program represents**. We use those numbers
//! to prioritize which aggregator processors to flesh out first.

use crate::metrics;
use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult, instruction::InstructionProcessorInputType, metrics::MetricsCollection,
    processor::Processor,
};
use std::{marker::PhantomData, sync::Arc};

pub struct AggWatch<T> {
    family: &'static str,
    _t: PhantomData<T>,
}

impl<T> AggWatch<T> {
    pub fn new(family: &'static str) -> Self {
        Self {
            family,
            _t: PhantomData,
        }
    }
}

#[async_trait]
impl<T: Send + Sync + 'static> Processor for AggWatch<T> {
    type InputType = InstructionProcessorInputType<T>;

    async fn process(
        &mut self,
        _data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        metrics::inc_decoded(self.family);
        Ok(())
    }
}
