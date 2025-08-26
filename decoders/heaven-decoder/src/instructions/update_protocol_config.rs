use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
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
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let protocol_config_state = next_account(&mut iter)?;
        let protocol_owner_state = next_account(&mut iter)?;

        Some(UpdateProtocolConfigInstructionAccounts {
            owner,
            protocol_config_state,
            protocol_owner_state,
        })
    }
}
