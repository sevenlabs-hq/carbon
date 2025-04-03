use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct DeprecatedCreateReservationList {}

pub struct DeprecatedCreateReservationListInstructionAccounts {
    pub reservation_list: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub master_edition: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedCreateReservationList {
    type ArrangedAccounts = DeprecatedCreateReservationListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [reservation_list, payer, update_authority, master_edition, resource, metadata, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
