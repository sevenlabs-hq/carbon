use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x35924408127511b9")]
pub struct DeleteTokenBadge {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeleteTokenBadgeInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpools_config_extension: solana_pubkey::Pubkey,
    pub token_badge_authority: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub token_badge: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeleteTokenBadge {
    type ArrangedAccounts = DeleteTokenBadgeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpools_config_extension = next_account(&mut iter)?;
        let token_badge_authority = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let token_badge = next_account(&mut iter)?;
        let receiver = next_account(&mut iter)?;

        Some(DeleteTokenBadgeInstructionAccounts {
            whirlpools_config,
            whirlpools_config_extension,
            token_badge_authority,
            token_mint,
            token_badge,
            receiver,
        })
    }
}
