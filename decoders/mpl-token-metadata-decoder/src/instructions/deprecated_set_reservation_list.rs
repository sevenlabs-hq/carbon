use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05")]
pub struct DeprecatedSetReservationList {}

pub struct DeprecatedSetReservationListInstructionAccounts {
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub reservation_list: solana_sdk::pubkey::Pubkey,
    pub resource: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedSetReservationList {
    type ArrangedAccounts = DeprecatedSetReservationListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [master_edition, reservation_list, resource, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeprecatedSetReservationListInstructionAccounts {
            master_edition: master_edition.pubkey,
            reservation_list: reservation_list.pubkey,
            resource: resource.pubkey,
        })
    }
}
