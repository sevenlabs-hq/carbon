
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x74ce1bbfa6130049")]
pub struct ClaimToken{
    pub id: u8,
}

pub struct ClaimTokenInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub program_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub associated_token_token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for ClaimToken {
    type ArrangedAccounts = ClaimTokenInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let wallet = accounts.get(1)?;
        let program_authority = accounts.get(2)?;
        let program_token_account = accounts.get(3)?;
        let destination_token_account = accounts.get(4)?;
        let mint = accounts.get(5)?;
        let associated_token_token_program = accounts.get(6)?;
        let associated_token_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(ClaimTokenInstructionAccounts {
            payer: *payer,
            wallet: *wallet,
            program_authority: *program_authority,
            program_token_account: *program_token_account,
            destination_token_account: *destination_token_account,
            mint: *mint,
            associated_token_token_program: *associated_token_token_program,
            associated_token_program: *associated_token_program,
            system_program: *system_program,
        })
    }
}