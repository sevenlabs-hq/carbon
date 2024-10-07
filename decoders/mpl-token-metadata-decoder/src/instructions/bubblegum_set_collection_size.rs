
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe6d7e7e29cbc3806")]
pub struct BubblegumSetCollectionSize{
    pub set_collection_size_args: SetCollectionSizeArgs,
}

pub struct BubblegumSetCollectionSizeInstructionAccounts {
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub bubblegum_signer: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for BubblegumSetCollectionSize {
    type ArrangedAccounts = BubblegumSetCollectionSizeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let collection_metadata = accounts.get(0)?;
        let collection_authority = accounts.get(1)?;
        let collection_mint = accounts.get(2)?;
        let bubblegum_signer = accounts.get(3)?;
        let collection_authority_record = accounts.get(4)?;

        Some(BubblegumSetCollectionSizeInstructionAccounts {
            collection_metadata: *collection_metadata,
            collection_authority: *collection_authority,
            collection_mint: *collection_mint,
            bubblegum_signer: *bubblegum_signer,
            collection_authority_record: *collection_authority_record,
        })
    }
}