
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6d7fba4e724125ec")]
pub struct UpdateRewardInfosEvent{
    pub reward_growth_global_x64: [u128; 3],
}
