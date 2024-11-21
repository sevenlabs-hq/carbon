
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xba8fd11dfe02c275")]
pub struct TwoHopSwapV2{
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

pub struct TwoHopSwapV2InstructionAccounts {
    pub whirlpool_one: solana_sdk::pubkey::Pubkey,
    pub whirlpool_two: solana_sdk::pubkey::Pubkey,
    pub token_mint_input: solana_sdk::pubkey::Pubkey,
    pub token_mint_intermediate: solana_sdk::pubkey::Pubkey,
    pub token_mint_output: solana_sdk::pubkey::Pubkey,
    pub token_program_input: solana_sdk::pubkey::Pubkey,
    pub token_program_intermediate: solana_sdk::pubkey::Pubkey,
    pub token_program_output: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_input: solana_sdk::pubkey::Pubkey,
    pub token_vault_one_input: solana_sdk::pubkey::Pubkey,
    pub token_vault_one_intermediate: solana_sdk::pubkey::Pubkey,
    pub token_vault_two_intermediate: solana_sdk::pubkey::Pubkey,
    pub token_vault_two_output: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_output: solana_sdk::pubkey::Pubkey,
    pub token_authority: solana_sdk::pubkey::Pubkey,
    pub tick_array_one0: solana_sdk::pubkey::Pubkey,
    pub tick_array_one1: solana_sdk::pubkey::Pubkey,
    pub tick_array_one2: solana_sdk::pubkey::Pubkey,
    pub tick_array_two0: solana_sdk::pubkey::Pubkey,
    pub tick_array_two1: solana_sdk::pubkey::Pubkey,
    pub tick_array_two2: solana_sdk::pubkey::Pubkey,
    pub oracle_one: solana_sdk::pubkey::Pubkey,
    pub oracle_two: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TwoHopSwapV2 {
    type ArrangedAccounts = TwoHopSwapV2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let whirlpool_one = accounts.get(0)?;
        let whirlpool_two = accounts.get(1)?;
        let token_mint_input = accounts.get(2)?;
        let token_mint_intermediate = accounts.get(3)?;
        let token_mint_output = accounts.get(4)?;
        let token_program_input = accounts.get(5)?;
        let token_program_intermediate = accounts.get(6)?;
        let token_program_output = accounts.get(7)?;
        let token_owner_account_input = accounts.get(8)?;
        let token_vault_one_input = accounts.get(9)?;
        let token_vault_one_intermediate = accounts.get(10)?;
        let token_vault_two_intermediate = accounts.get(11)?;
        let token_vault_two_output = accounts.get(12)?;
        let token_owner_account_output = accounts.get(13)?;
        let token_authority = accounts.get(14)?;
        let tick_array_one0 = accounts.get(15)?;
        let tick_array_one1 = accounts.get(16)?;
        let tick_array_one2 = accounts.get(17)?;
        let tick_array_two0 = accounts.get(18)?;
        let tick_array_two1 = accounts.get(19)?;
        let tick_array_two2 = accounts.get(20)?;
        let oracle_one = accounts.get(21)?;
        let oracle_two = accounts.get(22)?;
        let memo_program = accounts.get(23)?;

        Some(TwoHopSwapV2InstructionAccounts {
            whirlpool_one: whirlpool_one.pubkey,
            whirlpool_two: whirlpool_two.pubkey,
            token_mint_input: token_mint_input.pubkey,
            token_mint_intermediate: token_mint_intermediate.pubkey,
            token_mint_output: token_mint_output.pubkey,
            token_program_input: token_program_input.pubkey,
            token_program_intermediate: token_program_intermediate.pubkey,
            token_program_output: token_program_output.pubkey,
            token_owner_account_input: token_owner_account_input.pubkey,
            token_vault_one_input: token_vault_one_input.pubkey,
            token_vault_one_intermediate: token_vault_one_intermediate.pubkey,
            token_vault_two_intermediate: token_vault_two_intermediate.pubkey,
            token_vault_two_output: token_vault_two_output.pubkey,
            token_owner_account_output: token_owner_account_output.pubkey,
            token_authority: token_authority.pubkey,
            tick_array_one0: tick_array_one0.pubkey,
            tick_array_one1: tick_array_one1.pubkey,
            tick_array_one2: tick_array_one2.pubkey,
            tick_array_two0: tick_array_two0.pubkey,
            tick_array_two1: tick_array_two1.pubkey,
            tick_array_two2: tick_array_two2.pubkey,
            oracle_one: oracle_one.pubkey,
            oracle_two: oracle_two.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
