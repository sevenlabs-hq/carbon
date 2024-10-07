
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc6d4ab6d90d7ae59")]
pub struct WithdrawFees{
    pub amount: u64,
}

pub struct WithdrawFeesInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub program_fee_ata: solana_sdk::pubkey::Pubkey,
    pub admin_fee_ata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for WithdrawFees {
    type ArrangedAccounts = WithdrawFeesInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let fee_authority = accounts.get(2)?;
        let program_fee_ata = accounts.get(3)?;
        let admin_fee_ata = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let associated_token_program = accounts.get(7)?;

        Some(WithdrawFeesInstructionAccounts {
            admin: *admin,
            mint: *mint,
            fee_authority: *fee_authority,
            program_fee_ata: *program_fee_ata,
            admin_fee_ata: *admin_fee_ata,
            system_program: *system_program,
            token_program: *token_program,
            associated_token_program: *associated_token_program,
        })
    }
}