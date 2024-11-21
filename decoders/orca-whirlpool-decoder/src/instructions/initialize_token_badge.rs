use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfd4dcd5f1be059df")]
pub struct InitializeTokenBadge {}

pub struct InitializeTokenBadgeInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpools_config_extension: solana_sdk::pubkey::Pubkey,
    pub token_badge_authority: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_badge: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTokenBadge {
    type ArrangedAccounts = InitializeTokenBadgeInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let whirlpools_config_extension = accounts.get(1)?;
        let token_badge_authority = accounts.get(2)?;
        let token_mint = accounts.get(3)?;
        let token_badge = accounts.get(4)?;
        let funder = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(InitializeTokenBadgeInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            whirlpools_config_extension: whirlpools_config_extension.pubkey,
            token_badge_authority: token_badge_authority.pubkey,
            token_mint: token_mint.pubkey,
            token_badge: token_badge.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
