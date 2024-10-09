
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d908efc549d352579")]
pub struct PositionCreate{
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}
