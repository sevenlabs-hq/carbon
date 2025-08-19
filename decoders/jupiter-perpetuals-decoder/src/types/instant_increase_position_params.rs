use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InstantIncreasePositionParams {
    pub size_usd_delta: u64,
    pub collateral_token_delta: Option<u64>,
    pub side: Side,
    pub price_slippage: u64,
    pub request_time: i64,
}
