
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5fb40aac54aee828")]
pub struct InitializePool{
    pub bumps: WhirlpoolBumps,
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

pub struct InitializePoolInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub token_mint_a: solana_sdk::pubkey::Pubkey,
    pub token_mint_b: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub fee_tier: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializePool {
    type ArrangedAccounts = InitializePoolInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let token_mint_a = accounts.get(1)?;
        let token_mint_b = accounts.get(2)?;
        let funder = accounts.get(3)?;
        let whirlpool = accounts.get(4)?;
        let token_vault_a = accounts.get(5)?;
        let token_vault_b = accounts.get(6)?;
        let fee_tier = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let rent = accounts.get(10)?;

        Some(InitializePoolInstructionAccounts {
            whirlpools_config: *whirlpools_config,
            token_mint_a: *token_mint_a,
            token_mint_b: *token_mint_b,
            funder: *funder,
            whirlpool: *whirlpool,
            token_vault_a: *token_vault_a,
            token_vault_b: *token_vault_b,
            fee_tier: *fee_tier,
            token_program: *token_program,
            system_program: *system_program,
            rent: *rent,
        })
    }
}