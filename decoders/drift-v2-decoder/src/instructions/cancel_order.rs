use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f81edf00831df84")]
pub struct CancelOrder {
    pub order_id: Option<u32>,
}

pub struct CancelOrderInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelOrder {
    type ArrangedAccounts = CancelOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelOrderInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
