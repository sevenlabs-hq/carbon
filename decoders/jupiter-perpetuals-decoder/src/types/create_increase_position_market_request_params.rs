use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateIncreasePositionMarketRequestParams {
    pub size_usd_delta: u64,
    pub collateral_token_delta: u64,
    pub side: Side,
    pub price_slippage: u64,
    pub jupiter_minimum_out: Option<u64>,
    pub counter: u64,
}
