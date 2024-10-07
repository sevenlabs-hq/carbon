
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x370935097239d134")]
pub struct InitializeConfigExtension{
}

pub struct InitializeConfigExtensionInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub config_extension: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializeConfigExtension {
    type ArrangedAccounts = InitializeConfigExtensionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let config_extension = accounts.get(1)?;
        let funder = accounts.get(2)?;
        let fee_authority = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(InitializeConfigExtensionInstructionAccounts {
            config: *config,
            config_extension: *config_extension,
            funder: *funder,
            fee_authority: *fee_authority,
            system_program: *system_program,
        })
    }
}