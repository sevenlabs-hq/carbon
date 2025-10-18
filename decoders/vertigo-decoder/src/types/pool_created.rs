use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct PoolCreated {
    pub pool: solana_pubkey::Pubkey,
    pub mint_a: solana_pubkey::Pubkey,
    pub mint_b: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub initial_token_reserves: u64,
    pub shift: u128,
    pub fee_params: FeeParams,
}
