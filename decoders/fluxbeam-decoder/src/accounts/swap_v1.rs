use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapV1 {
    pub _padding: u8,
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub token_program_id: solana_pubkey::Pubkey,
    pub token_a: solana_pubkey::Pubkey,
    pub token_b: solana_pubkey::Pubkey,
    pub pool_mint: solana_pubkey::Pubkey,
    pub token_a_mint: solana_pubkey::Pubkey,
    pub token_b_mint: solana_pubkey::Pubkey,
    pub pool_fee_account: solana_pubkey::Pubkey,
    pub fees: Fees,
    pub swap_curve: SwapCurve,
}
