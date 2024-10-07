
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0d09d36b3eace043")]
pub struct InitFee{
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}

pub struct InitFeeInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitFee {
    type ArrangedAccounts = InitFeeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(InitFeeInstructionAccounts {
            keeper: *keeper,
            fee_authority: *fee_authority,
            system_program: *system_program,
        })
    }
}