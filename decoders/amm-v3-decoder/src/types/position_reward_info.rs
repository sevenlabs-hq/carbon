
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct PositionRewardInfo {
    pub growth_inside_last_x64: u128,
    pub reward_amount_owed: u64,
}
