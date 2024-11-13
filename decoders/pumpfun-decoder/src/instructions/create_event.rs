use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1b72a94ddeeb6376")]
pub struct CreateEvent {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
}
