
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xad73a54ee284cdfe")]
pub struct CreateVote{
}

pub struct CreateVoteInstructionAccounts {
    pub vote: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CreateVote {
    type ArrangedAccounts = CreateVoteInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let vote = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(CreateVoteInstructionAccounts {
            vote: vote.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}