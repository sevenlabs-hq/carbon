
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf8c69e91e17587c8")]
pub struct Swap{
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

pub struct SwapInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_authority: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array0: solana_sdk::pubkey::Pubkey,
    pub tick_array1: solana_sdk::pubkey::Pubkey,
    pub tick_array2: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let token_authority = accounts.get(1)?;
        let whirlpool = accounts.get(2)?;
        let token_owner_account_a = accounts.get(3)?;
        let token_vault_a = accounts.get(4)?;
        let token_owner_account_b = accounts.get(5)?;
        let token_vault_b = accounts.get(6)?;
        let tick_array0 = accounts.get(7)?;
        let tick_array1 = accounts.get(8)?;
        let tick_array2 = accounts.get(9)?;
        let oracle = accounts.get(10)?;

        Some(SwapInstructionAccounts {
            token_program: *token_program,
            token_authority: *token_authority,
            whirlpool: *whirlpool,
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