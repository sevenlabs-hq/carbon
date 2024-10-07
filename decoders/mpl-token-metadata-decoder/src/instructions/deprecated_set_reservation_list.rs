
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x441c42133bcbbe8e")]
pub struct DeprecatedSetReservationList{
    pub set_reservation_list_args: SetReservationListArgs,
}

pub struct DeprecatedSetReservationListInstructionAccounts {
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub reservation_list: solana_sdk::pubkey::Pubkey,
    pub resource: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DeprecatedSetReservationList {
    type ArrangedAccounts = DeprecatedSetReservationListInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let master_edition = accounts.get(0)?;
        let reservation_list = accounts.get(1)?;
        let resource = accounts.get(2)?;

        Some(DeprecatedSetReservationListInstructionAccounts {
            master_edition: *master_edition,
            reservation_list: *reservation_list,
            resource: *resource,
        })
    }
}