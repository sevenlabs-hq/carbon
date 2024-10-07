
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe87a7319c78f88a2")]
pub struct FillOrder{
    pub making_amount: u64,
    pub max_taking_amount: u64,
}

pub struct FillOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub taker_output_account: solana_sdk::pubkey::Pubkey,
    pub maker_output_account: solana_sdk::pubkey::Pubkey,
    pub taker_input_account: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub program_fee_account: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for FillOrder {
    type ArrangedAccounts = FillOrderInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let order = accounts.get(0)?;
        let reserve = accounts.get(1)?;
        let maker = accounts.get(2)?;
        let taker = accounts.get(3)?;
        let taker_output_account = accounts.get(4)?;
        let maker_output_account = accounts.get(5)?;
        let taker_input_account = accounts.get(6)?;
        let fee_authority = accounts.get(7)?;
        let program_fee_account = accounts.get(8)?;
        let referral = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;

        Some(FillOrderInstructionAccounts {
            order: *order,
            reserve: *reserve,
            maker: *maker,
            taker: *taker,
            taker_output_account: *taker_output_account,
            maker_output_account: *maker_output_account,
            taker_input_account: *taker_input_account,
            fee_authority: *fee_authority,
            program_fee_account: *program_fee_account,
            referral: *referral,
            token_program: *token_program,
            system_program: *system_program,
        })
    }
}