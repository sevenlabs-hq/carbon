
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3701195873431418")]
pub struct Unverify{
    pub verification_args: VerificationArgs,
}

pub struct UnverifyInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub delegate_record: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Unverify {
    type ArrangedAccounts = UnverifyInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let delegate_record = accounts.get(1)?;
        let metadata = accounts.get(2)?;
        let collection_mint = accounts.get(3)?;
        let collection_metadata = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let sysvar_instructions = accounts.get(6)?;

        Some(UnverifyInstructionAccounts {
            authority: *authority,
            delegate_record: *delegate_record,
            metadata: *metadata,
            collection_mint: *collection_mint,
            collection_metadata: *collection_metadata,
            system_program: *system_program,
            sysvar_instructions: *sysvar_instructions,
        })
    }
}