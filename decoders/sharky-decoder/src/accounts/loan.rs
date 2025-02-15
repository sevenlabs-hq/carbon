use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x14c34675a5e3b601")]
pub struct Loan {
    pub version: u8,
    pub principal_lamports: u64,
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub value_token_mint: solana_sdk::pubkey::Pubkey,
    pub escrow_bump_seed: u8,
    pub loan_state: LoanState,
}
