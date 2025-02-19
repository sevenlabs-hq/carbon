use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x35924408127511b9")]
pub struct DeleteTokenBadge {}

pub struct DeleteTokenBadgeInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpools_config_extension: solana_sdk::pubkey::Pubkey,
    pub token_badge_authority: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_badge: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeleteTokenBadge {
    type ArrangedAccounts = DeleteTokenBadgeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, whirlpools_config_extension, token_badge_authority, token_mint, token_badge, receiver, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DeleteTokenBadgeInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            whirlpools_config_extension: whirlpools_config_extension.pubkey,
            token_badge_authority: token_badge_authority.pubkey,
            token_mint: token_mint.pubkey,
            token_badge: token_badge.pubkey,
            receiver: receiver.pubkey,
        })
    }
}
