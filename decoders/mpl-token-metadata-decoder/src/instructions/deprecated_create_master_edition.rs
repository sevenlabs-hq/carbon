
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9b7fa59fec5c4f15")]
pub struct DeprecatedCreateMasterEdition{
    pub create_master_edition_args: CreateMasterEditionArgs,
}

pub struct DeprecatedCreateMasterEditionInstructionAccounts {
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
    pub one_time_printing_authorization_mint: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub printing_mint_authority: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub one_time_printing_authorization_mint_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DeprecatedCreateMasterEdition {
    type ArrangedAccounts = DeprecatedCreateMasterEditionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let edition = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let printing_mint = accounts.get(2)?;
        let one_time_printing_authorization_mint = accounts.get(3)?;
        let update_authority = accounts.get(4)?;
        let printing_mint_authority = accounts.get(5)?;
        let mint_authority = accounts.get(6)?;
        let metadata = accounts.get(7)?;
        let payer = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let rent = accounts.get(11)?;
        let one_time_printing_authorization_mint_authority = accounts.get(12)?;

        Some(DeprecatedCreateMasterEditionInstructionAccounts {
            edition: *edition,
            mint: *mint,
            printing_mint: *printing_mint,
            one_time_printing_authorization_mint: *one_time_printing_authorization_mint,
            update_authority: *update_authority,
            printing_mint_authority: *printing_mint_authority,
            mint_authority: *mint_authority,
            metadata: *metadata,
            payer: *payer,
            token_program: *token_program,
            system_program: *system_program,
            rent: *rent,
            one_time_printing_authorization_mint_authority: *one_time_printing_authorization_mint_authority,
        })
    }
}