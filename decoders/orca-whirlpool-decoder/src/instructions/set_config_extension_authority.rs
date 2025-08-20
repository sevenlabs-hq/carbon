use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2c5ef17418bc3c8f")]
pub struct SetConfigExtensionAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetConfigExtensionAuthorityInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpools_config_extension: solana_pubkey::Pubkey,
    pub config_extension_authority: solana_pubkey::Pubkey,
    pub new_config_extension_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetConfigExtensionAuthority {
    type ArrangedAccounts = SetConfigExtensionAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpools_config_extension = next_account(&mut iter)?;
        let config_extension_authority = next_account(&mut iter)?;
        let new_config_extension_authority = next_account(&mut iter)?;

        Some(SetConfigExtensionAuthorityInstructionAccounts {
            whirlpools_config,
            whirlpools_config_extension,
            config_extension_authority,
            new_config_extension_authority,
        })
    }
}
