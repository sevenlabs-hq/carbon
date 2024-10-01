
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb1ad2af0b8047c51")]
pub struct RaydiumSwap{
}

pub struct RaydiumSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub amm_id: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub serum_program_id: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub serum_event_queue: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub user_source_token_account: solana_sdk::pubkey::Pubkey,
    pub user_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub user_source_owner: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for RaydiumSwap {
    type ArrangedAccounts = RaydiumSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let token_program = accounts.get(1)?;
        let amm_id = accounts.get(2)?;
        let amm_authority = accounts.get(3)?;
        let amm_open_orders = accounts.get(4)?;
        let pool_coin_token_account = accounts.get(5)?;
        let pool_pc_token_account = accounts.get(6)?;
        let serum_program_id = accounts.get(7)?;
        let serum_market = accounts.get(8)?;
        let serum_bids = accounts.get(9)?;
        let serum_asks = accounts.get(10)?;
        let serum_event_queue = accounts.get(11)?;
        let serum_coin_vault_account = accounts.get(12)?;
        let serum_pc_vault_account = accounts.get(13)?;
        let serum_vault_signer = accounts.get(14)?;
        let user_source_token_account = accounts.get(15)?;
        let user_destination_token_account = accounts.get(16)?;
        let user_source_owner = accounts.get(17)?;

        Some(RaydiumSwapInstructionAccounts {
            swap_program: *swap_program,
            token_program: *token_program,
            amm_id: *amm_id,
            amm_authority: *amm_authority,
            amm_open_orders: *amm_open_orders,
            pool_coin_token_account: *pool_coin_token_account,
            pool_pc_token_account: *pool_pc_token_account,
            serum_program_id: *serum_program_id,
            serum_market: *serum_market,
            serum_bids: *serum_bids,
            serum_asks: *serum_asks,
            serum_event_queue: *serum_event_queue,
            serum_coin_vault_account: *serum_coin_vault_account,
            serum_pc_vault_account: *serum_pc_vault_account,
            serum_vault_signer: *serum_vault_signer,
            user_source_token_account: *user_source_token_account,
            user_destination_token_account: *user_destination_token_account,
            user_source_owner: *user_source_owner,
        })
    }
}