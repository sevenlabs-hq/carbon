use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb552130f7ecd98f2")]
pub struct SetProtocolSlotFees {
    pub version: u16,
    pub fee_type: FeeType,
    pub slot_fees: SlotFeeBracketsParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetProtocolSlotFeesInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub protocol_config_state: solana_pubkey::Pubkey,
    pub protocol_owner_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetProtocolSlotFees {
    type ArrangedAccounts = SetProtocolSlotFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let protocol_config_state = next_account(&mut iter)?;
        let protocol_owner_state = next_account(&mut iter)?;

        Some(SetProtocolSlotFeesInstructionAccounts {
            owner,
            protocol_config_state,
            protocol_owner_state,
        })
    }
}
