use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcfca0420cd4f0db2")]
pub struct SetTokenBadgeAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetTokenBadgeAuthorityInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpools_config_extension: solana_pubkey::Pubkey,
    pub config_extension_authority: solana_pubkey::Pubkey,
    pub new_token_badge_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTokenBadgeAuthority {
    type ArrangedAccounts = SetTokenBadgeAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpools_config_extension = next_account(&mut iter)?;
        let config_extension_authority = next_account(&mut iter)?;
        let new_token_badge_authority = next_account(&mut iter)?;

        Some(SetTokenBadgeAuthorityInstructionAccounts {
            whirlpools_config,
            whirlpools_config_extension,
            config_extension_authority,
            new_token_badge_authority,
        })
    }
}
