use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05")]
pub struct DeprecatedSetReservationList {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeprecatedSetReservationListInstructionAccounts {
    pub master_edition: solana_pubkey::Pubkey,
    pub reservation_list: solana_pubkey::Pubkey,
    pub resource: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeprecatedSetReservationList {
    type ArrangedAccounts = DeprecatedSetReservationListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let master_edition = next_account(&mut iter)?;
        let reservation_list = next_account(&mut iter)?;
        let resource = next_account(&mut iter)?;

        Some(DeprecatedSetReservationListInstructionAccounts {
            master_edition,
            reservation_list,
            resource,
        })
    }
}
