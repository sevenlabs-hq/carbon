use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0f")]
pub struct UpdateMetadataAccountV2 {
    pub update_metadata_account_args_v2: UpdateMetadataAccountArgsV2,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateMetadataAccountV2InstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateMetadataAccountV2 {
    type ArrangedAccounts = UpdateMetadataAccountV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;

        Some(UpdateMetadataAccountV2InstructionAccounts {
            metadata,
            update_authority,
        })
    }
}
