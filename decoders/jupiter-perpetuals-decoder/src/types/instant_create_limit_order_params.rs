use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InstantCreateLimitOrderParams {
    pub size_usd_delta: u64,
    pub collateral_token_delta: u64,
    pub side: Side,
    pub trigger_price: u64,
    pub trigger_above_threshold: bool,
    pub counter: u64,
    pub request_time: i64,
}
