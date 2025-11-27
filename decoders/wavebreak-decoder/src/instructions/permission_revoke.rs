use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05")]
pub struct PermissionRevoke {
    pub permission_message: PermissionMessage,
    pub permission_signature: PermissionSignature,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PermissionRevokeInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub permission_config: solana_pubkey::Pubkey,
    pub consumed_permission: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PermissionRevoke {
    type ArrangedAccounts = PermissionRevokeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let permission_config = next_account(&mut iter)?;
        let consumed_permission = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(PermissionRevokeInstructionAccounts {
            funder,
            permission_config,
            consumed_permission,
            system_program,
        })
    }
}
