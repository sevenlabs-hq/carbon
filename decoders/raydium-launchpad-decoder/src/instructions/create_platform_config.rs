use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

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
    pub cpswap_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub transfer_fee_extension_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePlatformConfig {
    type ArrangedAccounts = CreatePlatformConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let platform_admin = next_account(&mut iter)?;
        let platform_fee_wallet = next_account(&mut iter)?;
        let platform_nft_wallet = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let cpswap_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let transfer_fee_extension_authority = next_account(&mut iter)?;

        Some(CreatePlatformConfigInstructionAccounts {
            platform_admin,
            platform_fee_wallet,
            platform_nft_wallet,
            platform_config,
            cpswap_config,
            system_program,
            transfer_fee_extension_authority,
        })
    }
}
