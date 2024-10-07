
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xabe3a19e01b06948")]
pub struct DeprecatedCreateReservationList{
}

pub struct DeprecatedCreateReservationListInstructionAccounts {
    pub reservation_list: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub resource: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DeprecatedCreateReservationList {
    type ArrangedAccounts = DeprecatedCreateReservationListInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let reservation_list = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let update_authority = accounts.get(2)?;
        let master_edition = accounts.get(3)?;
        let resource = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent = accounts.get(7)?;

        Some(DeprecatedCreateReservationListInstructionAccounts {
            reservation_list: *reservation_list,
            payer: *payer,
            update_authority: *update_authority,
            master_edition: *master_edition,
            resource: *resource,
            metadata: *metadata,
            system_program: *system_program,
            rent: *rent,
        })
    }
}