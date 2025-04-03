use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x18")]
pub struct RevokeCollectionAuthority {}

pub struct RevokeCollectionAuthorityInstructionAccounts {
    pub collection_authority_record: solana_pubkey::Pubkey,
    pub delegate_authority: solana_pubkey::Pubkey,
    pub revoke_authority: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RevokeCollectionAuthority {
    type ArrangedAccounts = RevokeCollectionAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collection_authority_record, delegate_authority, revoke_authority, metadata, mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RevokeCollectionAuthorityInstructionAccounts {
            collection_authority_record: collection_authority_record.pubkey,
            delegate_authority: delegate_authority.pubkey,
            revoke_authority: revoke_authority.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
        })
    }
}
