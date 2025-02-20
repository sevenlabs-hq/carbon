use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Assets {
    pub fees_reserves: u64,
    pub owned: u64,
    pub locked: u64,
    pub guaranteed_usd: u64,
    pub global_short_sizes: u64,
    pub global_short_average_prices: u64,
}
