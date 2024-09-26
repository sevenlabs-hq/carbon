
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6dceb61764cb7ad3")]
pub struct UpdatePoolStatus{
    pub status: u8,
}

pub struct UpdatePoolStatusInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpdatePoolStatus {
    type ArrangedAccounts = UpdatePoolStatusInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let pool_state = accounts.get(1)?;

        Some(UpdatePoolStatusInstructionAccounts {
            authority: *authority,
            pool_state: *pool_state,
        })
    }
}