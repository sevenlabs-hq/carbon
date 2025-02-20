use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2af2426ae40a6f9c")]
pub struct TransferAdmin {
    pub params: TransferAdminParams,
}

pub struct TransferAdminInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub new_admin: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferAdmin {
    type ArrangedAccounts = TransferAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, new_admin, perpetuals, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TransferAdminInstructionAccounts {
            admin: admin.pubkey,
            new_admin: new_admin.pubkey,
            perpetuals: perpetuals.pubkey,
        })
    }
}
