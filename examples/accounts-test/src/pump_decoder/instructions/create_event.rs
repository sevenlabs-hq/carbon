
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1b72a94ddeeb6376")]
pub struct CreateEvent{
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
}
