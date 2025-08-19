use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x499fcdb12b557589")]
pub struct EditTriggerOrderV2 {
    pub order_price: u64,
    pub trigger_price: Option<u64>,
    pub trigger_direction: Option<TriggerDirection>,
    pub trigger_ts: Option<u64>,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub reduce_only: bool,
}

pub struct EditTriggerOrderV2InstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub trigger_order: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditTriggerOrderV2 {
    type ArrangedAccounts = EditTriggerOrderV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, trigger_order, state, margin_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EditTriggerOrderV2InstructionAccounts {
            owner: owner.pubkey,
            trigger_order: trigger_order.pubkey,
            state: state.pubkey,
            margin_account: margin_account.pubkey,
        })
    }
}
