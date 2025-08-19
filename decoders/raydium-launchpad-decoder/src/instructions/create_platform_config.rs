use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb05ac4affd71dc14")]
pub struct CreatePlatformConfig {
    pub platform_params: PlatformParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreatePlatformConfigInstructionAccounts {
    pub platform_admin: solana_pubkey::Pubkey,
    pub platform_fee_wallet: solana_pubkey::Pubkey,
    pub platform_nft_wallet: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePlatformConfig {
    type ArrangedAccounts = CreatePlatformConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [platform_admin, platform_fee_wallet, platform_nft_wallet, platform_config, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreatePlatformConfigInstructionAccounts {
            platform_admin: platform_admin.pubkey,
            platform_fee_wallet: platform_fee_wallet.pubkey,
            platform_nft_wallet: platform_nft_wallet.pubkey,
            platform_config: platform_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
