use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22")]
pub struct SetCollectionSize {
    pub set_collection_size_args: SetCollectionSizeArgs,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetCollectionSizeInstructionAccounts {
    pub collection_metadata: solana_pubkey::Pubkey,
    pub collection_authority: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection_authority_record: Option<solana_pubkey::Pubkey>,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollectionSize {
    type ArrangedAccounts = SetCollectionSizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let collection_metadata = next_account(&mut iter)?;
        let collection_authority = next_account(&mut iter)?;
        let collection_mint = next_account(&mut iter)?;
        let collection_authority_record = next_account(&mut iter);

        Some(SetCollectionSizeInstructionAccounts {
            collection_metadata,
            collection_authority,
            collection_mint,
            collection_authority_record,
        })
    }
}
