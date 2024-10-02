
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x333091737b5f478a")]
pub struct TokenSwapV2{
}

pub struct TokenSwapV2InstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub swap: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source: solana_sdk::pubkey::Pubkey,
    pub swap_source: solana_sdk::pubkey::Pubkey,
    pub swap_destination: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub pool_fee: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub source_token_program: solana_sdk::pubkey::Pubkey,
    pub destination_token_program: solana_sdk::pubkey::Pubkey,
    pub pool_token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for TokenSwapV2 {
    type ArrangedAccounts = TokenSwapV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let swap = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let user_transfer_authority = accounts.get(3)?;
        let source = accounts.get(4)?;
        let swap_source = accounts.get(5)?;
        let swap_destination = accounts.get(6)?;
        let destination = accounts.get(7)?;
        let pool_mint = accounts.get(8)?;
        let pool_fee = accounts.get(9)?;
        let source_mint = accounts.get(10)?;
        let destination_mint = accounts.get(11)?;
        let source_token_program = accounts.get(12)?;
        let destination_token_program = accounts.get(13)?;
        let pool_token_program = accounts.get(14)?;

        Some(TokenSwapV2InstructionAccounts {
            swap_program: *swap_program,
            swap: *swap,
            authority: *authority,
            user_transfer_authority: *user_transfer_authority,
            source: *source,
            swap_source: *swap_source,
            swap_destination: *swap_destination,
            destination: *destination,
            pool_mint: *pool_mint,
            pool_fee: *pool_fee,
            source_mint: *source_mint,
            destination_mint: *destination_mint,
            source_token_program: *source_token_program,
            destination_token_program: *destination_token_program,
            pool_token_program: *pool_token_program,
        })
    }
}