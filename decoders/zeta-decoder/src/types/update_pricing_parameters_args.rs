use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdatePricingParametersArgs {
    pub option_trade_normalizer: u64,
    pub future_trade_normalizer: u64,
    pub max_volatility_retreat: u64,
    pub max_interest_retreat: u64,
    pub min_delta: u64,
    pub max_delta: u64,
    pub min_interest_rate: i64,
    pub max_interest_rate: i64,
    pub min_volatility: u64,
    pub max_volatility: u64,
}
