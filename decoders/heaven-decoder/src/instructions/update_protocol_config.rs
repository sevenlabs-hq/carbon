use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0xc5617b36dda80b87")]
pub struct UpdateProtocolConfig {
    pub version: u16,
    pub params: ProtocolConfigParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateProtocolConfigInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub protocol_config_state: solana_pubkey::Pubkey,
    pub protocol_owner_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateProtocolConfig {
    type ArrangedAccounts = UpdateProtocolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, protocol_config_state, protocol_owner_state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateProtocolConfigInstructionAccounts {
            owner: owner.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
        })
    }
}
