use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb42bd770fe741485")]
pub struct EditTriggerOrder {
    pub order_price: u64,
    pub trigger_price: Option<u64>,
    pub trigger_direction: Option<TriggerDirection>,
    pub trigger_ts: Option<u64>,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub reduce_only: bool,
}

pub struct EditTriggerOrderInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub trigger_order: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditTriggerOrder {
    type ArrangedAccounts = EditTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, trigger_order, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EditTriggerOrderInstructionAccounts {
            owner: owner.pubkey,
            trigger_order: trigger_order.pubkey,
        })
    }
}
