
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd02fc29b116252ec")]
pub struct Collect{
}

pub struct CollectInstructionAccounts {
    pub recipient1: solana_sdk::pubkey::Pubkey,
    pub recipient2: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Collect {
    type ArrangedAccounts = CollectInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let recipient1 = accounts.get(0)?;
        let recipient2 = accounts.get(1)?;

        Some(CollectInstructionAccounts {
            recipient1: recipient1.pubkey,
            recipient2: recipient2.pubkey,
        })
    }
}