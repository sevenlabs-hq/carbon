
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xff5c572dc6acec02")]
pub struct PreInitialize{
    pub nonce: u8,
}

pub struct PreInitializeInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub pool_withdraw_queue: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub lp_mint_address: solana_sdk::pubkey::Pubkey,
    pub coin_mint_address: solana_sdk::pubkey::Pubkey,
    pub pc_mint_address: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_temp_lp_token_account: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub user_wallet: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for PreInitialize {
    type ArrangedAccounts = PreInitializeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let system_program = accounts.get(1)?;
        let rent = accounts.get(2)?;
        let amm_target_orders = accounts.get(3)?;
        let pool_withdraw_queue = accounts.get(4)?;
        let amm_authority = accounts.get(5)?;
        let lp_mint_address = accounts.get(6)?;
        let coin_mint_address = accounts.get(7)?;
        let pc_mint_address = accounts.get(8)?;
        let pool_coin_token_account = accounts.get(9)?;
        let pool_pc_token_account = accounts.get(10)?;
        let pool_temp_lp_token_account = accounts.get(11)?;
        let serum_market = accounts.get(12)?;
        let user_wallet = accounts.get(13)?;

        Some(PreInitializeInstructionAccounts {
            token_program: *token_program,
            system_program: *system_program,
            rent: *rent,
            amm_target_orders: *amm_target_orders,
            pool_withdraw_queue: *pool_withdraw_queue,
            amm_authority: *amm_authority,
            lp_mint_address: *lp_mint_address,
            coin_mint_address: *coin_mint_address,
            pc_mint_address: *pc_mint_address,
            pool_coin_token_account: *pool_coin_token_account,
            pool_pc_token_account: *pool_pc_token_account,
            pool_temp_lp_token_account: *pool_temp_lp_token_account,
            serum_market: *serum_market,
            user_wallet: *user_wallet,
        })
    }
}