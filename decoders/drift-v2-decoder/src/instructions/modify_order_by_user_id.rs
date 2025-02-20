use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9e4d04fdfcc2a1b3")]
pub struct ModifyOrderByUserId {
    pub user_order_id: u8,
    pub modify_order_params: ModifyOrderParams,
}

pub struct ModifyOrderByUserIdInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ModifyOrderByUserId {
    type ArrangedAccounts = ModifyOrderByUserIdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ModifyOrderByUserIdInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
