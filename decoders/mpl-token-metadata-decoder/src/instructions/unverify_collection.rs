use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x16")]
pub struct UnverifyCollection {}

pub struct UnverifyCollectionInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition_account: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnverifyCollection {
    type ArrangedAccounts = UnverifyCollectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, collection_authority, collection_mint, collection, collection_master_edition_account, collection_authority_record] =
            accounts
        else {
            return None;
        };

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
