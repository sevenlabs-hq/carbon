
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7be5b83f0c005c91")]
pub struct WhirlpoolSwap{
}

pub struct WhirlpoolSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
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

impl ArrangeAccounts for WhirlpoolSwap {
    type ArrangedAccounts = WhirlpoolSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let token_program = accounts.get(1)?;
        let token_authority = accounts.get(2)?;
        let whirlpool = accounts.get(3)?;
        let token_owner_account_a = accounts.get(4)?;
        let token_vault_a = accounts.get(5)?;
        let token_owner_account_b = accounts.get(6)?;
        let token_vault_b = accounts.get(7)?;
        let tick_array0 = accounts.get(8)?;
        let tick_array1 = accounts.get(9)?;
        let tick_array2 = accounts.get(10)?;
        let oracle = accounts.get(11)?;

        Some(WhirlpoolSwapInstructionAccounts {
            swap_program: *swap_program,
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