
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2c5ef17418bc3c8f")]
pub struct SetConfigExtensionAuthority{
}

pub struct SetConfigExtensionAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpools_config_extension: solana_sdk::pubkey::Pubkey,
    pub config_extension_authority: solana_sdk::pubkey::Pubkey,
    pub new_config_extension_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetConfigExtensionAuthority {
    type ArrangedAccounts = SetConfigExtensionAuthorityInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let whirlpools_config_extension = accounts.get(1)?;
        let config_extension_authority = accounts.get(2)?;
        let new_config_extension_authority = accounts.get(3)?;

        Some(SetConfigExtensionAuthorityInstructionAccounts {
            whirlpools_config: *whirlpools_config,
            whirlpools_config_extension: *whirlpools_config_extension,
            config_extension_authority: *config_extension_authority,
            new_config_extension_authority: *new_config_extension_authority,
        })
    }
}