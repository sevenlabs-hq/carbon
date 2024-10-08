
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1dd399583e953cb146")]
pub struct InitializeReward{
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub reward_index: u64,
    pub reward_duration: u64,
}
