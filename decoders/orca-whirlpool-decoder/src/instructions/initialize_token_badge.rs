use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfd4dcd5f1be059df")]
pub struct InitializeTokenBadge {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeTokenBadgeInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpools_config_extension: solana_pubkey::Pubkey,
    pub token_badge_authority: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub token_badge: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTokenBadge {
    type ArrangedAccounts = InitializeTokenBadgeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpools_config_extension = next_account(&mut iter)?;
        let token_badge_authority = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let token_badge = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeTokenBadgeInstructionAccounts {
            whirlpools_config,
            whirlpools_config_extension,
            token_badge_authority,
            token_mint,
            token_badge,
            funder,
            system_program,
        })
    }
}
