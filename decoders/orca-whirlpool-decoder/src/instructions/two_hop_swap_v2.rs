use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xba8fd11dfe02c275")]
pub struct TwoHopSwapV2 {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TwoHopSwapV2InstructionAccounts {
    pub whirlpool_one: solana_pubkey::Pubkey,
    pub whirlpool_two: solana_pubkey::Pubkey,
    pub token_mint_input: solana_pubkey::Pubkey,
    pub token_mint_intermediate: solana_pubkey::Pubkey,
    pub token_mint_output: solana_pubkey::Pubkey,
    pub token_program_input: solana_pubkey::Pubkey,
    pub token_program_intermediate: solana_pubkey::Pubkey,
    pub token_program_output: solana_pubkey::Pubkey,
    pub token_owner_account_input: solana_pubkey::Pubkey,
    pub token_vault_one_input: solana_pubkey::Pubkey,
    pub token_vault_one_intermediate: solana_pubkey::Pubkey,
    pub token_vault_two_intermediate: solana_pubkey::Pubkey,
    pub token_vault_two_output: solana_pubkey::Pubkey,
    pub token_owner_account_output: solana_pubkey::Pubkey,
    pub token_authority: solana_pubkey::Pubkey,
    pub tick_array_one0: solana_pubkey::Pubkey,
    pub tick_array_one1: solana_pubkey::Pubkey,
    pub tick_array_one2: solana_pubkey::Pubkey,
    pub tick_array_two0: solana_pubkey::Pubkey,
    pub tick_array_two1: solana_pubkey::Pubkey,
    pub tick_array_two2: solana_pubkey::Pubkey,
    pub oracle_one: solana_pubkey::Pubkey,
    pub oracle_two: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TwoHopSwapV2 {
    type ArrangedAccounts = TwoHopSwapV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpool_one = next_account(&mut iter)?;
        let whirlpool_two = next_account(&mut iter)?;
        let token_mint_input = next_account(&mut iter)?;
        let token_mint_intermediate = next_account(&mut iter)?;
        let token_mint_output = next_account(&mut iter)?;
        let token_program_input = next_account(&mut iter)?;
        let token_program_intermediate = next_account(&mut iter)?;
        let token_program_output = next_account(&mut iter)?;
        let token_owner_account_input = next_account(&mut iter)?;
        let token_vault_one_input = next_account(&mut iter)?;
        let token_vault_one_intermediate = next_account(&mut iter)?;
        let token_vault_two_intermediate = next_account(&mut iter)?;
        let token_vault_two_output = next_account(&mut iter)?;
        let token_owner_account_output = next_account(&mut iter)?;
        let token_authority = next_account(&mut iter)?;
        let tick_array_one0 = next_account(&mut iter)?;
        let tick_array_one1 = next_account(&mut iter)?;
        let tick_array_one2 = next_account(&mut iter)?;
        let tick_array_two0 = next_account(&mut iter)?;
        let tick_array_two1 = next_account(&mut iter)?;
        let tick_array_two2 = next_account(&mut iter)?;
        let oracle_one = next_account(&mut iter)?;
        let oracle_two = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;

        Some(TwoHopSwapV2InstructionAccounts {
            whirlpool_one,
            whirlpool_two,
            token_mint_input,
            token_mint_intermediate,
            token_mint_output,
            token_program_input,
            token_program_intermediate,
            token_program_output,
            token_owner_account_input,
            token_vault_one_input,
            token_vault_one_intermediate,
            token_vault_two_intermediate,
            token_vault_two_output,
            token_owner_account_output,
            token_authority,
            tick_array_one0,
            tick_array_one1,
            tick_array_one2,
            tick_array_two0,
            tick_array_two1,
            tick_array_two2,
            oracle_one,
            oracle_two,
            memo_program,
        })
    }
}
