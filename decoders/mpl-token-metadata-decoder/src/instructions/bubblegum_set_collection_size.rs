
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x24")]
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

impl carbon_core::deserialize::ArrangeAccounts for BubblegumSetCollectionSize {
    type ArrangedAccounts = BubblegumSetCollectionSizeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let collection_metadata = accounts.get(0)?;
        let collection_authority = accounts.get(1)?;
        let collection_mint = accounts.get(2)?;
        let bubblegum_signer = accounts.get(3)?;
        let collection_authority_record = accounts.get(4)?;

        Some(BubblegumSetCollectionSizeInstructionAccounts {
            collection_metadata: collection_metadata.pubkey,
            collection_authority: collection_authority.pubkey,
            collection_mint: collection_mint.pubkey,
            bubblegum_signer: bubblegum_signer.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
        })
    }
}