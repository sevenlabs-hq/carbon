use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Observation {
    pub block_timestamp: u64,
    pub cumulative_token0_price_x32: u128,
    pub cumulative_token1_price_x32: u128,
}
