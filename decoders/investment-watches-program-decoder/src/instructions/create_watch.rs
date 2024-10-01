
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2be4e66868cd76d0")]
pub struct CreateWatch{
}

pub struct CreateWatchInstructionAccounts {
    pub watch: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CreateWatch {
    type ArrangedAccounts = CreateWatchInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let watch = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let collection = accounts.get(2)?;
        let candy_machine = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(CreateWatchInstructionAccounts {
            watch: *watch,
            authority: *authority,
            collection: *collection,
            candy_machine: *candy_machine,
            system_program: *system_program,
        })
    }
}