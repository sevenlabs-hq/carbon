
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xbbc128792f4990b1")]
pub struct InvariantSwap{
}

pub struct InvariantSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub tickmap: solana_sdk::pubkey::Pubkey,
    pub account_x: solana_sdk::pubkey::Pubkey,
    pub account_y: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InvariantSwap {
    type ArrangedAccounts = InvariantSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let state = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let tickmap = accounts.get(3)?;
        let account_x = accounts.get(4)?;
        let account_y = accounts.get(5)?;
        let reserve_x = accounts.get(6)?;
        let reserve_y = accounts.get(7)?;
        let owner = accounts.get(8)?;
        let program_authority = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(InvariantSwapInstructionAccounts {
            swap_program: *swap_program,
            state: *state,
            pool: *pool,
            tickmap: *tickmap,
            account_x: *account_x,
            account_y: *account_y,
            reserve_x: *reserve_x,
            reserve_y: *reserve_y,
            owner: *owner,
            program_authority: *program_authority,
            token_program: *token_program,
        })
    }
}