
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfafb2a6a2989baa8")]
pub struct UnverifyCollection{
}

pub struct UnverifyCollectionInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition_account: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UnverifyCollection {
    type ArrangedAccounts = UnverifyCollectionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let collection_authority = accounts.get(1)?;
        let collection_mint = accounts.get(2)?;
        let collection = accounts.get(3)?;
        let collection_master_edition_account = accounts.get(4)?;
        let collection_authority_record = accounts.get(5)?;

        Some(UnverifyCollectionInstructionAccounts {
            metadata: metadata.pubkey,
            collection_authority: collection_authority.pubkey,
            collection_mint: collection_mint.pubkey,
            collection: collection.pubkey,
            collection_master_edition_account: collection_master_edition_account.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
        })
    }
}