use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x52689806956f640d")]
pub struct SetDecompressibleState {
    pub decompressable_state: DecompressibleState,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetDecompressibleStateInstructionAccounts {
    pub tree_authority: solana_pubkey::Pubkey,
    pub tree_creator: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetDecompressibleState {
    type ArrangedAccounts = SetDecompressibleStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [tree_authority, tree_creator, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetDecompressibleStateInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            tree_creator: tree_creator.pubkey,
        })
    }
}
