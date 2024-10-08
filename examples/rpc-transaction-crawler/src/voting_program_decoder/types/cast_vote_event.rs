
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CastVoteEvent {
    pub vote: solana_sdk::pubkey::Pubkey,
    pub voter: solana_sdk::pubkey::Pubkey,
    pub choice: bool,
}
