
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1db94afc7d1bd7bc6f")]
pub struct LbPairCreate{
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_step: u16,
    pub token_x: solana_sdk::pubkey::Pubkey,
    pub token_y: solana_sdk::pubkey::Pubkey,
}
