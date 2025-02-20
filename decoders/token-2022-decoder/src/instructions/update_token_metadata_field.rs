use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdde9312db5cadcc8")]
pub struct UpdateTokenMetadataField {
    pub field: TokenMetadataField,
    pub value: String,
}

pub struct UpdateTokenMetadataFieldInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateTokenMetadataField {
    type ArrangedAccounts = UpdateTokenMetadataFieldInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, update_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTokenMetadataFieldInstructionAccounts {
            metadata: metadata.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}
