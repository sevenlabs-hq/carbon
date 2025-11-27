use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InstantDecreasePositionParams {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub price_slippage: u64,
    pub entire_position: Option<bool>,
    pub request_time: i64,
}
