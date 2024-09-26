
use super::*;


#[derive(Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
pub struct InitializeRewardParam {
    pub open_time: u64,
    pub end_time: u64,
    pub emissions_per_second_x64: u128,
}
