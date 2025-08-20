use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf2d57f21b3fcc43")]
pub struct InitializePoolV2 {
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePoolV2InstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub token_mint_a: solana_pubkey::Pubkey,
    pub token_mint_b: solana_pubkey::Pubkey,
    pub token_badge_a: solana_pubkey::Pubkey,
    pub token_badge_b: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub fee_tier: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePoolV2 {
    type ArrangedAccounts = InitializePoolV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let token_mint_a = next_account(&mut iter)?;
        let token_mint_b = next_account(&mut iter)?;
        let token_badge_a = next_account(&mut iter)?;
        let token_badge_b = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let token_vault_a = next_account(&mut iter)?;
        let token_vault_b = next_account(&mut iter)?;
        let fee_tier = next_account(&mut iter)?;
        let token_program_a = next_account(&mut iter)?;
        let token_program_b = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(InitializePoolV2InstructionAccounts {
            whirlpools_config,
            token_mint_a,
            token_mint_b,
            token_badge_a,
            token_badge_b,
            funder,
            whirlpool,
            token_vault_a,
            token_vault_b,
            fee_tier,
            token_program_a,
            token_program_b,
            system_program,
            rent,
        })
    }
}
