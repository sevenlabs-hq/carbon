use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SplitPositionInfo {
    pub liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward0: u64,
    pub reward1: u64,
}
