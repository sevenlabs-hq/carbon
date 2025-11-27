use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct HaltStateV2 {
    pub halted: bool,
    pub timestamp: u64,
    pub spot_price: u64,
    pub market_cleaned: bool,
}
