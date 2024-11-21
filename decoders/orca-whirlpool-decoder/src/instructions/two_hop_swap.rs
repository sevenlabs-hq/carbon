use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc360ed6c44a2dbe6")]
pub struct TwoHopSwap {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
}

pub struct TwoHopSwapInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_authority: solana_sdk::pubkey::Pubkey,
    pub whirlpool_one: solana_sdk::pubkey::Pubkey,
    pub whirlpool_two: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_one_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_one_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_one_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_one_b: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_two_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_two_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_two_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_two_b: solana_sdk::pubkey::Pubkey,
    pub tick_array_one0: solana_sdk::pubkey::Pubkey,
    pub tick_array_one1: solana_sdk::pubkey::Pubkey,
    pub tick_array_one2: solana_sdk::pubkey::Pubkey,
    pub tick_array_two0: solana_sdk::pubkey::Pubkey,
    pub tick_array_two1: solana_sdk::pubkey::Pubkey,
    pub tick_array_two2: solana_sdk::pubkey::Pubkey,
    pub oracle_one: solana_sdk::pubkey::Pubkey,
    pub oracle_two: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TwoHopSwap {
    type ArrangedAccounts = TwoHopSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let token_authority = accounts.get(1)?;
        let whirlpool_one = accounts.get(2)?;
        let whirlpool_two = accounts.get(3)?;
        let token_owner_account_one_a = accounts.get(4)?;
        let token_vault_one_a = accounts.get(5)?;
        let token_owner_account_one_b = accounts.get(6)?;
        let token_vault_one_b = accounts.get(7)?;
        let token_owner_account_two_a = accounts.get(8)?;
        let token_vault_two_a = accounts.get(9)?;
        let token_owner_account_two_b = accounts.get(10)?;
        let token_vault_two_b = accounts.get(11)?;
        let tick_array_one0 = accounts.get(12)?;
        let tick_array_one1 = accounts.get(13)?;
        let tick_array_one2 = accounts.get(14)?;
        let tick_array_two0 = accounts.get(15)?;
        let tick_array_two1 = accounts.get(16)?;
        let tick_array_two2 = accounts.get(17)?;
        let oracle_one = accounts.get(18)?;
        let oracle_two = accounts.get(19)?;

        Some(TwoHopSwapInstructionAccounts {
            token_program: token_program.pubkey,
            token_authority: token_authority.pubkey,
            whirlpool_one: whirlpool_one.pubkey,
            whirlpool_two: whirlpool_two.pubkey,
            token_owner_account_one_a: token_owner_account_one_a.pubkey,
            token_vault_one_a: token_vault_one_a.pubkey,
            token_owner_account_one_b: token_owner_account_one_b.pubkey,
            token_vault_one_b: token_vault_one_b.pubkey,
            token_owner_account_two_a: token_owner_account_two_a.pubkey,
            token_vault_two_a: token_vault_two_a.pubkey,
            token_owner_account_two_b: token_owner_account_two_b.pubkey,
            token_vault_two_b: token_vault_two_b.pubkey,
            tick_array_one0: tick_array_one0.pubkey,
            tick_array_one1: tick_array_one1.pubkey,
            tick_array_one2: tick_array_one2.pubkey,
            tick_array_two0: tick_array_two0.pubkey,
            tick_array_two1: tick_array_two1.pubkey,
            tick_array_two2: tick_array_two2.pubkey,
            oracle_one: oracle_one.pubkey,
            oracle_two: oracle_two.pubkey,
        })
    }
}
