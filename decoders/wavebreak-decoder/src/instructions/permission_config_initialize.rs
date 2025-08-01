use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct PermissionConfigInitialize {
    pub permission_authority: PermissionSigner,
    pub consumer_program: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PermissionConfigInitializeInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub permission_config: solana_pubkey::Pubkey,
    pub authority_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PermissionConfigInitialize {
    type ArrangedAccounts = PermissionConfigInitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let permission_config = next_account(&mut iter)?;
        let authority_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(PermissionConfigInitializeInstructionAccounts {
            authority,
            permission_config,
            authority_config,
            system_program,
        })
    }
}
