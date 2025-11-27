use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeZetaPricingArgs {
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub perp_impact_cash_delta: u64,
    pub margin_initial: u64,
    pub margin_maintenance: u64,
    pub pricing_nonce: u8,
}
