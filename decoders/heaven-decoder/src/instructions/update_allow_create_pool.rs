use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdffce73e60dbf1d6")]
pub struct UpdateAllowCreatePool {
    pub version: u16,
    pub allow_create_pool: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateAllowCreatePoolInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub protocol_config_state: solana_pubkey::Pubkey,
    pub protocol_admin_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAllowCreatePool {
    type ArrangedAccounts = UpdateAllowCreatePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, protocol_config_state, protocol_admin_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateAllowCreatePoolInstructionAccounts {
            admin: admin.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            protocol_admin_state: protocol_admin_state.pubkey,
        })
    }
}
