use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x16")]
pub struct UnverifyCollection {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UnverifyCollectionInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub collection_authority: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub collection_master_edition_account: solana_pubkey::Pubkey,
    pub collection_authority_record: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for UnverifyCollection {
    type ArrangedAccounts = UnverifyCollectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let collection_authority = next_account(&mut iter)?;
        let collection_mint = next_account(&mut iter)?;
        let collection = next_account(&mut iter)?;
        let collection_master_edition_account = next_account(&mut iter)?;
        let collection_authority_record = next_account(&mut iter);

        Some(UnverifyCollectionInstructionAccounts {
            metadata,
            collection_authority,
            collection_mint,
            collection,
            collection_master_edition_account,
            collection_authority_record,
        })
    }
}
