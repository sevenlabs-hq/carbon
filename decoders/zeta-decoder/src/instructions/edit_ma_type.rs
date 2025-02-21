use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe7d03332de934c4e")]
pub struct EditMaType {
    pub ma_type: MarginAccountType,
}

pub struct EditMaTypeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditMaType {
    type ArrangedAccounts = EditMaTypeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, margin_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EditMaTypeInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            margin_account: margin_account.pubkey,
        })
    }
}
