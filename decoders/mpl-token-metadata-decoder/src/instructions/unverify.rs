
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x35")]
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

impl carbon_core::deserialize::ArrangeAccounts for Unverify {
    type ArrangedAccounts = UnverifyInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let delegate_record = accounts.get(1)?;
        let metadata = accounts.get(2)?;
        let collection_mint = accounts.get(3)?;
        let collection_metadata = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let sysvar_instructions = accounts.get(6)?;

        Some(UnverifyInstructionAccounts {
            authority: authority.pubkey,
            delegate_record: delegate_record.pubkey,
            metadata: metadata.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
        })
    }
}