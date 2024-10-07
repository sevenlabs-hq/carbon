
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x38ade6d0ade49ccd")]
pub struct SwapWithPriceImpact{
    pub amount_in: u64,
    pub active_id: Option<i32>,
    pub max_price_impact_bps: u16,
}

pub struct SwapWithPriceImpactInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub user_token_in: solana_sdk::pubkey::Pubkey,
    pub user_token_out: solana_sdk::pubkey::Pubkey,
    pub token_x_mint: solana_sdk::pubkey::Pubkey,
    pub token_y_mint: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub host_fee_in: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub token_x_program: solana_sdk::pubkey::Pubkey,
    pub token_y_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SwapWithPriceImpact {
    type ArrangedAccounts = SwapWithPriceImpactInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let bin_array_bitmap_extension = accounts.get(1)?;
        let reserve_x = accounts.get(2)?;
        let reserve_y = accounts.get(3)?;
        let user_token_in = accounts.get(4)?;
        let user_token_out = accounts.get(5)?;
        let token_x_mint = accounts.get(6)?;
        let token_y_mint = accounts.get(7)?;
        let oracle = accounts.get(8)?;
        let host_fee_in = accounts.get(9)?;
        let user = accounts.get(10)?;
        let token_x_program = accounts.get(11)?;
        let token_y_program = accounts.get(12)?;
        let event_authority = accounts.get(13)?;
        let program = accounts.get(14)?;

        Some(SwapWithPriceImpactInstructionAccounts {
            lb_pair: *lb_pair,
            bin_array_bitmap_extension: *bin_array_bitmap_extension,
            reserve_x: *reserve_x,
            reserve_y: *reserve_y,
            user_token_in: *user_token_in,
            user_token_out: *user_token_out,
            token_x_mint: *token_x_mint,
            token_y_mint: *token_y_mint,
            oracle: *oracle,
            host_fee_in: *host_fee_in,
            user: *user,
            token_x_program: *token_x_program,
            token_y_program: *token_y_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}