
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x09cbfe405920b39f")]
pub struct Initialize2{
    pub nonce: u8,
    pub open_time: u64,
    pub init_pc_amount: u64,
    pub init_coin_amount: u64,
}

pub struct Initialize2InstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub spl_associated_token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub coin_mint: solana_sdk::pubkey::Pubkey,
    pub pc_mint: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_withdraw_queue: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub pool_temp_lp: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub user_wallet: solana_sdk::pubkey::Pubkey,
    pub user_token_coin: solana_sdk::pubkey::Pubkey,
    pub user_token_pc: solana_sdk::pubkey::Pubkey,
    pub user_lp_token_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Initialize2 {
    type ArrangedAccounts = Initialize2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let spl_associated_token_account = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let rent = accounts.get(3)?;
        let amm = accounts.get(4)?;
        let amm_authority = accounts.get(5)?;
        let amm_open_orders = accounts.get(6)?;
        let lp_mint = accounts.get(7)?;
        let coin_mint = accounts.get(8)?;
        let pc_mint = accounts.get(9)?;
        let pool_coin_token_account = accounts.get(10)?;
        let pool_pc_token_account = accounts.get(11)?;
        let pool_withdraw_queue = accounts.get(12)?;
        let amm_target_orders = accounts.get(13)?;
        let pool_temp_lp = accounts.get(14)?;
        let serum_program = accounts.get(15)?;
        let serum_market = accounts.get(16)?;
        let user_wallet = accounts.get(17)?;
        let user_token_coin = accounts.get(18)?;
        let user_token_pc = accounts.get(19)?;
        let user_lp_token_account = accounts.get(20)?;

        Some(Initialize2InstructionAccounts {
            token_program: *token_program,
            spl_associated_token_account: *spl_associated_token_account,
            system_program: *system_program,
            rent: *rent,
            amm: *amm,
            amm_authority: *amm_authority,
            amm_open_orders: *amm_open_orders,
            lp_mint: *lp_mint,
            coin_mint: *coin_mint,
            pc_mint: *pc_mint,
            pool_coin_token_account: *pool_coin_token_account,
            pool_pc_token_account: *pool_pc_token_account,
            pool_withdraw_queue: *pool_withdraw_queue,
            amm_target_orders: *amm_target_orders,
            pool_temp_lp: *pool_temp_lp,
            serum_program: *serum_program,
            serum_market: *serum_market,
            user_wallet: *user_wallet,
            user_token_coin: *user_token_coin,
            user_token_pc: *user_token_pc,
            user_lp_token_account: *user_lp_token_account,
        })
    }
}