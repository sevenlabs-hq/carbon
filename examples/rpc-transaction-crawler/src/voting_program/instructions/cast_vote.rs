
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

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

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let vote = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let vote_receipt = accounts.get(2)?;
        let voter = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(CastVoteInstructionAccounts {
            vote: *vote,
            authority: *authority,
            vote_receipt: *vote_receipt,
            voter: *voter,
            system_program: *system_program,
        })
    }
}