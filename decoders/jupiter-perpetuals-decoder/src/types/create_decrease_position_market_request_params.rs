use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateDecreasePositionMarketRequestParams {
    pub collateral_usd_delta: u64,
    pub size_usd_delta: u64,
    pub price_slippage: u64,
    pub jupiter_minimum_out: Option<u64>,
    pub entire_position: Option<bool>,
    pub counter: u64,
}
