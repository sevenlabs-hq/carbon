use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x18")]
pub struct RevokeCollectionAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
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
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let collection_authority_record = next_account(&mut iter)?;
        let delegate_authority = next_account(&mut iter)?;
        let revoke_authority = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;

        Some(RevokeCollectionAuthorityInstructionAccounts {
            collection_authority_record,
            delegate_authority,
            revoke_authority,
            metadata,
            mint,
        })
    }
}
