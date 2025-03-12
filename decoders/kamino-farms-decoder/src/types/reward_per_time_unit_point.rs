

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RewardPerTimeUnitPoint {
    pub ts_start: u64,
    pub reward_per_time_unit: u64,
}
