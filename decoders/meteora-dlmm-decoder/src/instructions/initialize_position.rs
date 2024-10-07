
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdbc0ea47bebf6650")]
pub struct InitializePosition{
    pub lower_bin_id: i32,
    pub width: i32,
}

pub struct InitializePositionInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializePosition {
    type ArrangedAccounts = InitializePositionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let position = accounts.get(1)?;
        let lb_pair = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let rent = accounts.get(5)?;
        let event_authority = accounts.get(6)?;
        let program = accounts.get(7)?;

        Some(InitializePositionInstructionAccounts {
            payer: *payer,
            position: *position,
            lb_pair: *lb_pair,
            owner: *owner,
            system_program: *system_program,
            rent: *rent,
            event_authority: *event_authority,
            program: *program,
        })
    }
}