
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9bc150795b93febb")]
pub struct InitiateDlmmFill{
}

pub struct InitiateDlmmFillInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub dca: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub keeper_in_ata: solana_sdk::pubkey::Pubkey,
    pub in_ata: solana_sdk::pubkey::Pubkey,
    pub out_ata: solana_sdk::pubkey::Pubkey,
    pub instructions_sysvar: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitiateDlmmFill {
    type ArrangedAccounts = InitiateDlmmFillInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let dca = accounts.get(1)?;
        let input_mint = accounts.get(2)?;
        let keeper_in_ata = accounts.get(3)?;
        let in_ata = accounts.get(4)?;
        let out_ata = accounts.get(5)?;
        let instructions_sysvar = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;

        Some(InitiateDlmmFillInstructionAccounts {
            keeper: *keeper,
            dca: *dca,
            input_mint: *input_mint,
            keeper_in_ata: *keeper_in_ata,
            in_ata: *in_ata,
            out_ata: *out_ata,
            instructions_sysvar: *instructions_sysvar,
            system_program: *system_program,
            token_program: *token_program,
            associated_token_program: *associated_token_program,
        })
    }
}