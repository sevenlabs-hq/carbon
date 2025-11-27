use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PricingParams {
    pub trade_impact_fee_scalar: u64,
    pub buffer: u64,
    pub swap_spread: u64,
    pub max_leverage: u64,
    pub max_global_long_sizes: u64,
    pub max_global_short_sizes: u64,
}
