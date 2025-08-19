use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00c0e902fcfb82a9")]
pub struct CancelOrderHalted {
    pub side: Side,
    pub order_id: u128,
    pub asset: Asset,
}

pub struct CancelOrderHaltedInstructionAccounts {
    pub cancel_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelOrderHalted {
    type ArrangedAccounts = CancelOrderHaltedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cancel_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelOrderHaltedInstructionAccounts {
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
