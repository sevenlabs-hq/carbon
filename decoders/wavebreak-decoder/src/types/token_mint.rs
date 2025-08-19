use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenMint {
    pub mint_authority_flag: u32,
    pub mint_authority: solana_pubkey::Pubkey,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority_flag: u32,
    pub freeze_authority: solana_pubkey::Pubkey,
}
