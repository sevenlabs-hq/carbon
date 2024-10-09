
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x14d40fbd45b44597")]
pub struct CastVote{
    pub choice: bool,
}

pub struct CastVoteInstructionAccounts {
    pub vote: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub vote_receipt: solana_sdk::pubkey::Pubkey,
    pub voter: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CastVote {
    type ArrangedAccounts = CastVoteInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let vote = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let vote_receipt = accounts.get(2)?;
        let voter = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(CastVoteInstructionAccounts {
            vote: vote.pubkey,
            authority: authority.pubkey,
            vote_receipt: vote_receipt.pubkey,
            voter: voter.pubkey,
            system_program: system_program.pubkey,
        })
    }
}