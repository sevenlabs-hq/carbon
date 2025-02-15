use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d195e4b2f7063353f")]
pub struct PoolCreatedEvent {
    pub token_mint0: solana_sdk::pubkey::Pubkey,
    pub token_mint1: solana_sdk::pubkey::Pubkey,
    pub tick_spacing: u16,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub sqrt_price_x64: u128,
    pub tick: i32,
    pub token_vault0: solana_sdk::pubkey::Pubkey,
    pub token_vault1: solana_sdk::pubkey::Pubkey,
}
