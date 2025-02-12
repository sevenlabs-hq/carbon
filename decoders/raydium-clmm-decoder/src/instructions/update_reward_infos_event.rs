use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6d7fba4e724125ec")]
pub struct UpdateRewardInfosEvent {
    pub reward_growth_global_x64: [u128; 3],
}
