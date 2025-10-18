use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FeeBracket {
    pub market_cap_upper_bound: u64,
    pub buy_fee_bps: u32,
    pub sell_fee_bps: u32,
}
