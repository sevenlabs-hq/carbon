use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaa7c802830698b94")]
pub struct CreateOrUpdateProtocolOwner {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateOrUpdateProtocolOwnerInstructionAccounts {
    pub system_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub current_owner: solana_pubkey::Pubkey,
    pub new_owner: solana_pubkey::Pubkey,
    pub protocol_owner_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrUpdateProtocolOwner {
    type ArrangedAccounts = CreateOrUpdateProtocolOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let system_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let current_owner = next_account(&mut iter)?;
        let new_owner = next_account(&mut iter)?;
        let protocol_owner_state = next_account(&mut iter)?;

        Some(CreateOrUpdateProtocolOwnerInstructionAccounts {
            system_program,
            payer,
            current_owner,
            new_owner,
            protocol_owner_state,
        })
    }
}
