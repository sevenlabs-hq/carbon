use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PerpParameters {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub impact_cash_delta: u64,
}
