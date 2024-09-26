
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x729628592ce5906f")]
pub struct SwapRouterBaseIn{
    pub amount_in: u64,
    pub amount_out_minimum: u64,
}

pub struct SwapRouterBaseInInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SwapRouterBaseIn {
    type ArrangedAccounts = SwapRouterBaseInInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let input_token_account = accounts.get(1)?;
        let input_token_mint = accounts.get(2)?;
        let token_program = accounts.get(3)?;
        let token_program2022 = accounts.get(4)?;
        let memo_program = accounts.get(5)?;

        Some(SwapRouterBaseInInstructionAccounts {
            payer: *payer,
            input_token_account: *input_token_account,
            input_token_mint: *input_token_mint,
            token_program: *token_program,
            token_program2022: *token_program2022,
            memo_program: *memo_program,
        })
    }
}