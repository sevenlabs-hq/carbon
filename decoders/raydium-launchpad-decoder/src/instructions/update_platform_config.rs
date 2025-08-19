use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

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
        let [platform_admin, platform_config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePlatformConfigInstructionAccounts {
            platform_admin: platform_admin.pubkey,
            platform_config: platform_config.pubkey,
        })
    }
}
