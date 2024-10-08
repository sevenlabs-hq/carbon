
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1de0b2ae4afca555b4")]
pub struct UpdateRewardFunder{
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub reward_index: u64,
    pub old_funder: solana_sdk::pubkey::Pubkey,
    pub new_funder: solana_sdk::pubkey::Pubkey,
}
