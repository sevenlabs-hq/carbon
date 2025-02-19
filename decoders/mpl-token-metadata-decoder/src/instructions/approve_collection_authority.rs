use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17")]
pub struct ApproveCollectionAuthority {}

pub struct ApproveCollectionAuthorityInstructionAccounts {
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub new_collection_authority: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ApproveCollectionAuthority {
    type ArrangedAccounts = ApproveCollectionAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collection_authority_record, new_collection_authority, update_authority, payer, metadata, mint, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ApproveCollectionAuthorityInstructionAccounts {
            collection_authority_record: collection_authority_record.pubkey,
            new_collection_authority: new_collection_authority.pubkey,
            update_authority: update_authority.pubkey,
            payer: payer.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
