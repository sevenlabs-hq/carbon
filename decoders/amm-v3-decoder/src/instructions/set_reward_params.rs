
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd0d2c89245dc0d86")]
pub struct SetRewardParams{
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
    pub open_time: u64,
    pub end_time: u64,
}

pub struct SetRewardParamsInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub operation_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetRewardParams {
    type ArrangedAccounts = SetRewardParamsInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let amm_config = accounts.get(1)?;
        let pool_state = accounts.get(2)?;
        let operation_state = accounts.get(3)?;
        let token_program = accounts.get(4)?;
        let token_program2022 = accounts.get(5)?;

        Some(SetRewardParamsInstructionAccounts {
            authority: *authority,
            amm_config: *amm_config,
            pool_state: *pool_state,
            operation_state: *operation_state,
            token_program: *token_program,
            token_program2022: *token_program2022,
        })
    }
}