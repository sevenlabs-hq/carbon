use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d80275edde6de7f8d")]
pub struct PoolUpdatedEvent {
    pub pubkey: solana_sdk::pubkey::Pubkey,
    pub data: PoolUpdatedData,
}
