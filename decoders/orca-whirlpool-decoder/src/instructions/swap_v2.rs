
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2b04ed0b1ac91e62")]
pub struct SwapV2{
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

pub struct SwapV2InstructionAccounts {
    pub token_program_a: solana_sdk::pubkey::Pubkey,
    pub token_program_b: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub token_authority: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_mint_a: solana_sdk::pubkey::Pubkey,
    pub token_mint_b: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array0: solana_sdk::pubkey::Pubkey,
    pub tick_array1: solana_sdk::pubkey::Pubkey,
    pub tick_array2: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SwapV2 {
    type ArrangedAccounts = SwapV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program_a = accounts.get(0)?;
        let token_program_b = accounts.get(1)?;
        let memo_program = accounts.get(2)?;
        let token_authority = accounts.get(3)?;
        let whirlpool = accounts.get(4)?;
        let token_mint_a = accounts.get(5)?;
        let token_mint_b = accounts.get(6)?;
        let token_owner_account_a = accounts.get(7)?;
        let token_vault_a = accounts.get(8)?;
        let token_owner_account_b = accounts.get(9)?;
        let token_vault_b = accounts.get(10)?;
        let tick_array0 = accounts.get(11)?;
        let tick_array1 = accounts.get(12)?;
        let tick_array2 = accounts.get(13)?;
        let oracle = accounts.get(14)?;

        Some(SwapV2InstructionAccounts {
            token_program_a: *token_program_a,
            token_program_b: *token_program_b,
            memo_program: *memo_program,
            token_authority: *token_authority,
            whirlpool: *whirlpool,
            token_mint_a: *token_mint_a,
            token_mint_b: *token_mint_b,
            token_owner_account_a: *token_owner_account_a,
            token_vault_a: *token_vault_a,
            token_owner_account_b: *token_owner_account_b,
            token_vault_b: *token_vault_b,
            tick_array0: *tick_array0,
            tick_array1: *tick_array1,
            tick_array2: *tick_array2,
            oracle: *oracle,
        })
    }
}