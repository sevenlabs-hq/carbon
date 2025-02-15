use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeRewardParam {
    pub open_time: u64,
    pub end_time: u64,
    pub emissions_per_second_x64: u128,
}
