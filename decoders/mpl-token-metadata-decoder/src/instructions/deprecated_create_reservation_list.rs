use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct DeprecatedCreateReservationList {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
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
        let mut iter = accounts.iter();
        let reservation_list = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let update_authority = next_account(&mut iter)?;
        let master_edition = next_account(&mut iter)?;
        let resource = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(DeprecatedCreateReservationListInstructionAccounts {
            reservation_list,
            payer,
            update_authority,
            master_edition,
            resource,
            metadata,
            system_program,
            rent,
        })
    }
}
