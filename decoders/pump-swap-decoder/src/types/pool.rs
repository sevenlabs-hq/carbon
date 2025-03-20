use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Pool {
    pub pool_bump: u8,
    pub index: u16,
    pub creator: solana_sdk::pubkey::Pubkey,
    pub base_mint: solana_sdk::pubkey::Pubkey,
    pub quote_mint: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub pool_base_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_quote_token_account: solana_sdk::pubkey::Pubkey,
    pub lp_supply: u64,
}
