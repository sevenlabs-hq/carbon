use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenAccount {
    pub mint: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub amount: u64,
    pub delegate_flag: u32,
    pub delegate: solana_pubkey::Pubkey,
    pub state: TokenAccountState,
    pub is_native_flag: u32,
    pub native_amount: u64,
    pub delegate_amount: u64,
    pub close_authority_flag: u32,
    pub close_authority: solana_pubkey::Pubkey,
}
