
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0f843b32c706fb2e")]
pub struct MigratePosition{
}

pub struct MigratePositionInstructionAccounts {
    pub position_v2: solana_sdk::pubkey::Pubkey,
    pub position_v1: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_lower: solana_sdk::pubkey::Pubkey,
    pub bin_array_upper: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent_receiver: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MigratePosition {
    type ArrangedAccounts = MigratePositionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let position_v2 = accounts.get(0)?;
        let position_v1 = accounts.get(1)?;
        let lb_pair = accounts.get(2)?;
        let bin_array_lower = accounts.get(3)?;
        let bin_array_upper = accounts.get(4)?;
        let owner = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent_receiver = accounts.get(7)?;
        let event_authority = accounts.get(8)?;
        let program = accounts.get(9)?;

        Some(MigratePositionInstructionAccounts {
            position_v2: *position_v2,
            position_v1: *position_v1,
            lb_pair: *lb_pair,
            bin_array_lower: *bin_array_lower,
            bin_array_upper: *bin_array_upper,
            owner: *owner,
            system_program: *system_program,
            rent_receiver: *rent_receiver,
            event_authority: *event_authority,
            program: *program,
        })
    }
}