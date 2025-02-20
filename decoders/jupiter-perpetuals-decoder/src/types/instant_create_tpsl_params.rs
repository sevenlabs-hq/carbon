use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InstantCreateTpslParams {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub trigger_price: u64,
    pub trigger_above_threshold: bool,
    pub entire_position: bool,
    pub counter: u64,
    pub request_time: i64,
}
