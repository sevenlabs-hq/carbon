use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x387165fd4f377aa9")]
pub struct VerifyCollection {}

pub struct VerifyCollectionInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition_account: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for VerifyCollection {
    type ArrangedAccounts = VerifyCollectionInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let collection_authority = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let collection_mint = accounts.get(3)?;
        let collection = accounts.get(4)?;
        let collection_master_edition_account = accounts.get(5)?;
        let collection_authority_record = accounts.get(6)?;

        Some(VerifyCollectionInstructionAccounts {
            metadata: metadata.pubkey,
            collection_authority: collection_authority.pubkey,
            payer: payer.pubkey,
            collection_mint: collection_mint.pubkey,
            collection: collection.pubkey,
            collection_master_edition_account: collection_master_edition_account.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
        })
    }
}
