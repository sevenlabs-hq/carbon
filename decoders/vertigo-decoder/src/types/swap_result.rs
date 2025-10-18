use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapResult {
    pub new_reserves_a: u128,
    pub new_reserves_b: u128,
    pub amount_a: u64,
    pub amount_b: u64,
    pub fee_a: u64,
}
