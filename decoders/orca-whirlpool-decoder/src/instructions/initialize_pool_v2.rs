use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf2d57f21b3fcc43")]
pub struct InitializePoolV2 {
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

pub struct InitializePoolV2InstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub token_mint_a: solana_sdk::pubkey::Pubkey,
    pub token_mint_b: solana_sdk::pubkey::Pubkey,
    pub token_badge_a: solana_sdk::pubkey::Pubkey,
    pub token_badge_b: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub fee_tier: solana_sdk::pubkey::Pubkey,
    pub token_program_a: solana_sdk::pubkey::Pubkey,
    pub token_program_b: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePoolV2 {
    type ArrangedAccounts = InitializePoolV2InstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let token_mint_a = accounts.get(1)?;
        let token_mint_b = accounts.get(2)?;
        let token_badge_a = accounts.get(3)?;
        let token_badge_b = accounts.get(4)?;
        let funder = accounts.get(5)?;
        let whirlpool = accounts.get(6)?;
        let token_vault_a = accounts.get(7)?;
        let token_vault_b = accounts.get(8)?;
        let fee_tier = accounts.get(9)?;
        let token_program_a = accounts.get(10)?;
        let token_program_b = accounts.get(11)?;
        let system_program = accounts.get(12)?;
        let rent = accounts.get(13)?;

        Some(InitializePoolV2InstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            token_mint_a: token_mint_a.pubkey,
            token_mint_b: token_mint_b.pubkey,
            token_badge_a: token_badge_a.pubkey,
            token_badge_b: token_badge_b.pubkey,
            funder: funder.pubkey,
            whirlpool: whirlpool.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_vault_b: token_vault_b.pubkey,
            fee_tier: fee_tier.pubkey,
            token_program_a: token_program_a.pubkey,
            token_program_b: token_program_b.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
