
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf92b9c84f217c7a0")]
pub struct TransferRewardOwner{
    pub new_owner: solana_sdk::pubkey::Pubkey,
}

pub struct TransferRewardOwnerInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for TransferRewardOwner {
    type ArrangedAccounts = TransferRewardOwnerInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let pool_state = accounts.get(1)?;

        Some(TransferRewardOwnerInstructionAccounts {
            authority: *authority,
            pool_state: *pool_state,
        })
    }
}