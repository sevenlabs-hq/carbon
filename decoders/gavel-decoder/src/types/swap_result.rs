use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapResult {
    pub side: Side,
    pub base_matched: u64,
    pub quote_matched: u64,
    pub base_matched_as_limit_order: u64,
    pub quote_matched_as_limit_order: u64,
    pub base_matched_as_swap: u64,
    pub quote_matched_as_swap: u64,
    pub fee_in_quote: u64,
}
