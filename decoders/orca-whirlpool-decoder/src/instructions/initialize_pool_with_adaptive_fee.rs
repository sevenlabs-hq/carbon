use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8f5e604cac7c77c7")]
pub struct InitializePoolWithAdaptiveFee {
    pub initial_sqrt_price: u128,
    pub trade_enable_timestamp: Option<u64>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializePoolWithAdaptiveFeeInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub token_mint_a: solana_pubkey::Pubkey,
    pub token_mint_b: solana_pubkey::Pubkey,
    pub token_badge_a: solana_pubkey::Pubkey,
    pub token_badge_b: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub initialize_pool_authority: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub adaptive_fee_tier: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePoolWithAdaptiveFee {
    type ArrangedAccounts = InitializePoolWithAdaptiveFeeInstructionAccounts;

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
        let initialize_pool_authority = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let oracle = next_account(&mut iter)?;
        let token_vault_a = next_account(&mut iter)?;
        let token_vault_b = next_account(&mut iter)?;
        let adaptive_fee_tier = next_account(&mut iter)?;
        let token_program_a = next_account(&mut iter)?;
        let token_program_b = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(InitializePoolWithAdaptiveFeeInstructionAccounts {
            whirlpools_config,
            token_mint_a,
            token_mint_b,
            token_badge_a,
            token_badge_b,
            funder,
            initialize_pool_authority,
            whirlpool,
            oracle,
            token_vault_a,
            token_vault_b,
            adaptive_fee_tier,
            token_program_a,
            token_program_b,
            system_program,
            rent,
        })
    }
}
