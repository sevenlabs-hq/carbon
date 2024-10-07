
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf02f99440dbee12a")]
pub struct PreFlashFillOrder{
    pub making_amount: u64,
}

pub struct PreFlashFillOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub taker_output_account: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_mint_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for PreFlashFillOrder {
    type ArrangedAccounts = PreFlashFillOrderInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let order = accounts.get(0)?;
        let reserve = accounts.get(1)?;
        let taker = accounts.get(2)?;
        let taker_output_account = accounts.get(3)?;
        let input_mint = accounts.get(4)?;
        let input_mint_token_program = accounts.get(5)?;
        let instruction = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(PreFlashFillOrderInstructionAccounts {
            order: *order,
            reserve: *reserve,
            taker: *taker,
            taker_output_account: *taker_output_account,
            input_mint: *input_mint,
            input_mint_token_program: *input_mint_token_program,
            instruction: *instruction,
            system_program: *system_program,
        })
    }
}