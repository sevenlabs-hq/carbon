use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateStandardLiquidityPoolEvent {
    pub pool_id: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub config_version: u16,
    pub initial_token_reserve: u64,
    pub initial_virtual_wsol_reserve: u64,
}
