
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d304cf17590d7f22c")]
pub struct FeeParameterUpdate{
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub protocol_share: u16,
    pub base_factor: u16,
}
