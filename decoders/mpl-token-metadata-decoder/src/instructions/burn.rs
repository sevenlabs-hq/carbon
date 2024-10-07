
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x746e1d386bdb2a5d")]
pub struct Burn{
    pub burn_args: BurnArgs,
}

pub struct BurnInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub master_edition_mint: solana_sdk::pubkey::Pubkey,
    pub master_edition_token: solana_sdk::pubkey::Pubkey,
    pub edition_marker: solana_sdk::pubkey::Pubkey,
    pub token_record: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Burn {
    type ArrangedAccounts = BurnInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let collection_metadata = accounts.get(1)?;
        let metadata = accounts.get(2)?;
        let edition = accounts.get(3)?;
        let mint = accounts.get(4)?;
        let token = accounts.get(5)?;
        let master_edition = accounts.get(6)?;
        let master_edition_mint = accounts.get(7)?;
        let master_edition_token = accounts.get(8)?;
        let edition_marker = accounts.get(9)?;
        let token_record = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let sysvar_instructions = accounts.get(12)?;
        let spl_token_program = accounts.get(13)?;

        Some(BurnInstructionAccounts {
            authority: *authority,
            collection_metadata: *collection_metadata,
            metadata: *metadata,
            edition: *edition,
            mint: *mint,
            token: *token,
            master_edition: *master_edition,
            master_edition_mint: *master_edition_mint,
            master_edition_token: *master_edition_token,
            edition_marker: *edition_marker,
            token_record: *token_record,
            system_program: *system_program,
            sysvar_instructions: *sysvar_instructions,
            spl_token_program: *spl_token_program,
        })
    }
}