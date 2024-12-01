

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x05")]
pub struct DeprecatedSetReservationList{
}

pub struct DeprecatedSetReservationListInstructionAccounts {
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub reservation_list: solana_sdk::pubkey::Pubkey,
    pub resource: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedSetReservationList {
    type ArrangedAccounts = DeprecatedSetReservationListInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let master_edition = accounts.get(0)?;
        let reservation_list = accounts.get(1)?;
        let resource = accounts.get(2)?;

        Some(DeprecatedSetReservationListInstructionAccounts {
            master_edition: master_edition.pubkey,
            reservation_list: reservation_list.pubkey,
            resource: resource.pubkey,
        })
    }
}