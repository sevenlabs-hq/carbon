

use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xeb84d7e1d52b2b26")]
pub struct UpdateProgramVersion{
    pub version: u8,
}

pub struct UpdateProgramVersionInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub program_version: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpdateProgramVersion {
    type ArrangedAccounts = UpdateProgramVersionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let program_version = accounts.get(1)?;

        Some(UpdateProgramVersionInstructionAccounts {
            authority: authority.pubkey,
            program_version: program_version.pubkey,
        })
    }
}