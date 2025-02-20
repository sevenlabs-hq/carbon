use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xf4feb86108bbf3b3")]
pub struct SwapV1 {
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub token_program_id: solana_sdk::pubkey::Pubkey,
    pub token_a: solana_sdk::pubkey::Pubkey,
    pub token_b: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub pool_fee_account: solana_sdk::pubkey::Pubkey,
    pub fees: Fees,
    pub swap_curve: SwapCurve,
}
