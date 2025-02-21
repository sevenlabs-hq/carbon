use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dac5272cf1b67d304")]
pub struct PoolBalanceUpdatedEvent {
    pub pubkey: solana_sdk::pubkey::Pubkey,
    pub data: PoolBalanceUpdatedData,
}
