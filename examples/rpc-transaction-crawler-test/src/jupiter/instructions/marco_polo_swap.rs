
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf1935e0f3a6cb344")]
pub struct MarcoPoloSwap{
}

pub struct MarcoPoloSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub token_x: solana_sdk::pubkey::Pubkey,
    pub token_y: solana_sdk::pubkey::Pubkey,
    pub pool_x_account: solana_sdk::pubkey::Pubkey,
    pub pool_y_account: solana_sdk::pubkey::Pubkey,
    pub swapper_x_account: solana_sdk::pubkey::Pubkey,
    pub swapper_y_account: solana_sdk::pubkey::Pubkey,
    pub swapper: solana_sdk::pubkey::Pubkey,
    pub referrer_x_account: solana_sdk::pubkey::Pubkey,
    pub referrer_y_account: solana_sdk::pubkey::Pubkey,
    pub referrer: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MarcoPoloSwap {
    type ArrangedAccounts = MarcoPoloSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let state = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let token_x = accounts.get(3)?;
        let token_y = accounts.get(4)?;
        let pool_x_account = accounts.get(5)?;
        let pool_y_account = accounts.get(6)?;
        let swapper_x_account = accounts.get(7)?;
        let swapper_y_account = accounts.get(8)?;
        let swapper = accounts.get(9)?;
        let referrer_x_account = accounts.get(10)?;
        let referrer_y_account = accounts.get(11)?;
        let referrer = accounts.get(12)?;
        let program_authority = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let associated_token_program = accounts.get(16)?;
        let rent = accounts.get(17)?;

        Some(MarcoPoloSwapInstructionAccounts {
            swap_program: *swap_program,
            state: *state,
            pool: *pool,
            token_x: *token_x,
            token_y: *token_y,
            pool_x_account: *pool_x_account,
            pool_y_account: *pool_y_account,
            swapper_x_account: *swapper_x_account,
            swapper_y_account: *swapper_y_account,
            swapper: *swapper,
            referrer_x_account: *referrer_x_account,
            referrer_y_account: *referrer_y_account,
            referrer: *referrer,
            program_authority: *program_authority,
            system_program: *system_program,
            token_program: *token_program,
            associated_token_program: *associated_token_program,
            rent: *rent,
        })
    }
}