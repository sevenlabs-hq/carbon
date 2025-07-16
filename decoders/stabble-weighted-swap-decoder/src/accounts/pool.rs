use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf19a6d0411b16dbc")]
pub struct Pool {
    pub owner: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub authority_bump: u8,
    pub is_active: bool,
    pub invariant: u64,
    pub swap_fee: u64,
    pub tokens: Vec<PoolToken>,
    pub pending_owner: Option<solana_pubkey::Pubkey>,
    pub max_supply: u64,
}
