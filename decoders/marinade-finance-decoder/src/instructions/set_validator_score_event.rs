
use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3a35edb2ee99559c")]
pub struct SetValidatorScoreEvent{
    pub state: solana_sdk::pubkey::Pubkey,
    pub validator: solana_sdk::pubkey::Pubkey,
    pub index: u32,
    pub score_change: U32ValueChange,
}
