
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb2f5fdcdecfae9d1")]
pub struct SignMetadata{
}

pub struct SignMetadataInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub creator: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SignMetadata {
    type ArrangedAccounts = SignMetadataInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let creator = accounts.get(1)?;

        Some(SignMetadataInstructionAccounts {
            metadata: metadata.pubkey,
            creator: creator.pubkey,
        })
    }
}