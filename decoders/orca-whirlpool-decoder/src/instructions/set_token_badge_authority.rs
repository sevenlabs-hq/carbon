use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcfca0420cd4f0db2")]
pub struct SetTokenBadgeAuthority {}

pub struct SetTokenBadgeAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpools_config_extension: solana_sdk::pubkey::Pubkey,
    pub config_extension_authority: solana_sdk::pubkey::Pubkey,
    pub new_token_badge_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenBadgeAuthority {
    type ArrangedAccounts = SetTokenBadgeAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, whirlpools_config_extension, config_extension_authority, new_token_badge_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetTokenBadgeAuthorityInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            whirlpools_config_extension: whirlpools_config_extension.pubkey,
            config_extension_authority: config_extension_authority.pubkey,
            new_token_badge_authority: new_token_badge_authority.pubkey,
        })
    }
}
