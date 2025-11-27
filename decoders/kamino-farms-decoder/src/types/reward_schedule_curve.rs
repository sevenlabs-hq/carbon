use {
    super::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RewardScheduleCurve {
    pub points: [RewardPerTimeUnitPoint; 20],
}
