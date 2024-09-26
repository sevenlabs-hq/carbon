
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1eb007affd96b892")]
pub struct UpdateRewardInfos{
}

pub struct UpdateRewardInfosInstructionAccounts {
    pub pool_state: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpdateRewardInfos {
    type ArrangedAccounts = UpdateRewardInfosInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let pool_state = accounts.get(0)?;

        Some(UpdateRewardInfosInstructionAccounts {
            pool_state: *pool_state,
        })
    }
}