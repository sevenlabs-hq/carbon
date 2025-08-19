use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateDecreasePositionRequest2Params {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub request_type: RequestType,
    pub price_slippage: Option<u64>,
    pub jupiter_minimum_out: Option<u64>,
    pub trigger_price: Option<u64>,
    pub trigger_above_threshold: Option<bool>,
    pub entire_position: Option<bool>,
    pub counter: u64,
}
