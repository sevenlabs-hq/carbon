use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenInfo {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub decimals: u64,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub padding: [u64; 6],
}
