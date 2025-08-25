use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6be1a5ed5b9ed5dc")]
pub struct EvtUpdatePoolCreatorEvent {
    pub pool: Pubkey,
    pub creator: Pubkey,
    pub new_creator: Pubkey,
}
