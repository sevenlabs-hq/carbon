use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct PermissionConfigUpdate {
    pub update: PermissionConfigUpdateType,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PermissionConfigUpdateInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub permission_config: solana_pubkey::Pubkey,
    pub authority_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PermissionConfigUpdate {
    type ArrangedAccounts = PermissionConfigUpdateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let permission_config = next_account(&mut iter)?;
        let authority_config = next_account(&mut iter)?;

        Some(PermissionConfigUpdateInstructionAccounts {
            authority,
            permission_config,
            authority_config,
        })
    }
}
