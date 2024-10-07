
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3a7fbc3e4f52c460")]
pub struct DecreaseLiquidityV2{
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

pub struct DecreaseLiquidityV2InstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_program_a: solana_sdk::pubkey::Pubkey,
    pub token_program_b: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint_a: solana_sdk::pubkey::Pubkey,
    pub token_mint_b: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DecreaseLiquidityV2 {
    type ArrangedAccounts = DecreaseLiquidityV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let token_program_a = accounts.get(1)?;
        let token_program_b = accounts.get(2)?;
        let memo_program = accounts.get(3)?;
        let position_authority = accounts.get(4)?;
        let position = accounts.get(5)?;
        let position_token_account = accounts.get(6)?;
        let token_mint_a = accounts.get(7)?;
        let token_mint_b = accounts.get(8)?;
        let token_owner_account_a = accounts.get(9)?;
        let token_owner_account_b = accounts.get(10)?;
        let token_vault_a = accounts.get(11)?;
        let token_vault_b = accounts.get(12)?;
        let tick_array_lower = accounts.get(13)?;
        let tick_array_upper = accounts.get(14)?;

        Some(DecreaseLiquidityV2InstructionAccounts {
            whirlpool: *whirlpool,
            token_program_a: *token_program_a,
            token_program_b: *token_program_b,
            memo_program: *memo_program,
            position_authority: *position_authority,
            position: *position,
            position_token_account: *position_token_account,
            token_mint_a: *token_mint_a,
            token_mint_b: *token_mint_b,
            token_owner_account_a: *token_owner_account_a,
            token_owner_account_b: *token_owner_account_b,
            token_vault_a: *token_vault_a,
            token_vault_b: *token_vault_b,
            tick_array_lower: *tick_array_lower,
            tick_array_upper: *tick_array_upper,
        })
    }
}