use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapEvent {
    pub swap_sequence_number: u64,
    pub pre_base_liquidity: u64,
    pub pre_quote_liquidity: u64,
    pub post_base_liquidity: u64,
    pub post_quote_liquidity: u64,
    pub snapshot_base_liquidity: u64,
    pub snapshot_quote_liquidity: u64,
    pub swap_result: SwapResult,
}
