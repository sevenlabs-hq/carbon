

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x06")]
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

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedCreateReservationList {
    type ArrangedAccounts = DeprecatedCreateReservationListInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let reservation_list = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let update_authority = accounts.get(2)?;
        let master_edition = accounts.get(3)?;
        let resource = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent = accounts.get(7)?;

        Some(DeprecatedCreateReservationListInstructionAccounts {
            reservation_list: reservation_list.pubkey,
            payer: payer.pubkey,
            update_authority: update_authority.pubkey,
            master_edition: master_edition.pubkey,
            resource: resource.pubkey,
            metadata: metadata.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}