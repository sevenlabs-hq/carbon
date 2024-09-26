
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x383b49f48795858c")]
pub struct UpdateOperationAccount{
    pub param: u8,
    pub keys: Vec<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateOperationAccountInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub operation_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpdateOperationAccount {
    type ArrangedAccounts = UpdateOperationAccountInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let operation_state = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(UpdateOperationAccountInstructionAccounts {
            owner: *owner,
            operation_state: *operation_state,
            system_program: *system_program,
        })
    }
}