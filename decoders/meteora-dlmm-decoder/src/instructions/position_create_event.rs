use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d908efc549d352579")]
pub struct PositionCreateEvent {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}
