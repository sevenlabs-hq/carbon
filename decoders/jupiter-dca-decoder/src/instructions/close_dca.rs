
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x16072162a8b722f3")]
pub struct CloseDca{
}

pub struct CloseDcaInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub dca: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub in_ata: solana_sdk::pubkey::Pubkey,
    pub out_ata: solana_sdk::pubkey::Pubkey,
    pub user_in_ata: solana_sdk::pubkey::Pubkey,
    pub user_out_ata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CloseDca {
    type ArrangedAccounts = CloseDcaInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let user = accounts.get(0)?;
        let dca = accounts.get(1)?;
        let input_mint = accounts.get(2)?;
        let output_mint = accounts.get(3)?;
        let in_ata = accounts.get(4)?;
        let out_ata = accounts.get(5)?;
        let user_in_ata = accounts.get(6)?;
        let user_out_ata = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;

        Some(CloseDcaInstructionAccounts {
            user: *user,
            dca: *dca,
            input_mint: *input_mint,
            output_mint: *output_mint,
            in_ata: *in_ata,
            out_ata: *out_ata,
            user_in_ata: *user_in_ata,
            user_out_ata: *user_out_ata,
            system_program: *system_program,
            token_program: *token_program,
            associated_token_program: *associated_token_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}