use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc33c4c81922d438f")]
pub struct UpdatePlatformConfig {
    pub param: PlatformConfigParam,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdatePlatformConfigInstructionAccounts {
    pub platform_admin: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePlatformConfig {
    type ArrangedAccounts = UpdatePlatformConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let platform_admin = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;

        Some(UpdatePlatformConfigInstructionAccounts {
            platform_admin,
            platform_config,
        })
    }
}
