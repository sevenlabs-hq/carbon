use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct PermissionConsumeTopLevel {
    pub permission_message: PermissionMessage,
    pub permission_signature: PermissionSignature,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PermissionConsumeTopLevelInstructionAccounts {
    pub consumer: solana_pubkey::Pubkey,
    pub permission_config: solana_pubkey::Pubkey,
    pub consumed_permission: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub instructions: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PermissionConsumeTopLevel {
    type ArrangedAccounts = PermissionConsumeTopLevelInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let consumer = next_account(&mut iter)?;
        let permission_config = next_account(&mut iter)?;
        let consumed_permission = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let instructions = next_account(&mut iter)?;

        Some(PermissionConsumeTopLevelInstructionAccounts {
            consumer,
            permission_config,
            consumed_permission,
            system_program,
            instructions,
        })
    }
}
