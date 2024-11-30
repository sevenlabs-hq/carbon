

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x18")]
pub struct RevokeCollectionAuthority{
}

pub struct RevokeCollectionAuthorityInstructionAccounts {
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub delegate_authority: solana_sdk::pubkey::Pubkey,
    pub revoke_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RevokeCollectionAuthority {
    type ArrangedAccounts = RevokeCollectionAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let collection_authority_record = accounts.get(0)?;
        let delegate_authority = accounts.get(1)?;
        let revoke_authority = accounts.get(2)?;
        let metadata = accounts.get(3)?;
        let mint = accounts.get(4)?;

        Some(RevokeCollectionAuthorityInstructionAccounts {
            collection_authority_record: collection_authority_record.pubkey,
            delegate_authority: delegate_authority.pubkey,
            revoke_authority: revoke_authority.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
        })
    }
}