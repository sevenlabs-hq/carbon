use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x157eb014556f351f")]
pub struct CreateOrUpdateProtocolFeeAdmin {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateOrUpdateProtocolFeeAdminInstructionAccounts {
    pub system_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub current_owner: solana_pubkey::Pubkey,
    pub protocol_owner_state: solana_pubkey::Pubkey,
    pub new_admin: solana_pubkey::Pubkey,
    pub protocol_fee_admin_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrUpdateProtocolFeeAdmin {
    type ArrangedAccounts = CreateOrUpdateProtocolFeeAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let system_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let current_owner = next_account(&mut iter)?;
        let protocol_owner_state = next_account(&mut iter)?;
        let new_admin = next_account(&mut iter)?;
        let protocol_fee_admin_state = next_account(&mut iter)?;

        Some(CreateOrUpdateProtocolFeeAdminInstructionAccounts {
            system_program,
            payer,
            current_owner,
            protocol_owner_state,
            new_admin,
            protocol_fee_admin_state,
        })
    }
}
