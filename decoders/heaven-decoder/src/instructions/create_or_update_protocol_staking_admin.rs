use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04acc4d578321e89")]
pub struct CreateOrUpdateProtocolStakingAdmin {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateOrUpdateProtocolStakingAdminInstructionAccounts {
    pub system_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub current_owner: solana_pubkey::Pubkey,
    pub protocol_owner_state: solana_pubkey::Pubkey,
    pub new_admin: solana_pubkey::Pubkey,
    pub protocol_staking_admin_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrUpdateProtocolStakingAdmin {
    type ArrangedAccounts = CreateOrUpdateProtocolStakingAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [system_program, payer, current_owner, protocol_owner_state, new_admin, protocol_staking_admin_state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateOrUpdateProtocolStakingAdminInstructionAccounts {
            system_program: system_program.pubkey,
            payer: payer.pubkey,
            current_owner: current_owner.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
            new_admin: new_admin.pubkey,
            protocol_staking_admin_state: protocol_staking_admin_state.pubkey,
        })
    }
}
