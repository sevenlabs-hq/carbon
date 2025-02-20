use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct JumpRateState {
    pub min_rate_bps: u64,
    pub max_rate_bps: u64,
    pub target_rate_bps: u64,
    pub target_utilization_rate: u64,
}
