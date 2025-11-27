use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct Pool {
    pub enabled: bool,
    pub owner: solana_pubkey::Pubkey,
    pub mint_a: solana_pubkey::Pubkey,
    pub mint_b: solana_pubkey::Pubkey,
    pub token_a_reserves: u128,
    pub token_b_reserves: u128,
    pub shift: u128,
    pub royalties: u64,
    pub vertigo_fees: u64,
    pub bump: u8,
    pub fee_params: FeeParams,
}
