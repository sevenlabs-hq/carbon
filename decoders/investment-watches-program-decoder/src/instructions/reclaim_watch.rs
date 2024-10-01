
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xcfd7c4fe82195502")]
pub struct ReclaimWatch{
}

pub struct ReclaimWatchInstructionAccounts {
    pub watch: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub redeem_token_account: solana_sdk::pubkey::Pubkey,
    pub authority_token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for ReclaimWatch {
    type ArrangedAccounts = ReclaimWatchInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let watch = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let redeem_token_account = accounts.get(2)?;
        let authority_token_account = accounts.get(3)?;
        let token_mint = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(ReclaimWatchInstructionAccounts {
            watch: *watch,
            authority: *authority,
            redeem_token_account: *redeem_token_account,
            authority_token_account: *authority_token_account,
            token_mint: *token_mint,
            system_program: *system_program,
            token_program: *token_program,
        })
    }
}