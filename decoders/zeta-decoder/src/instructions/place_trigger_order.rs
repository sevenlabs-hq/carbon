use {
    super::super::types::*,
    alloc::string::String,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x209c32bce89f70ec")]
pub struct PlaceTriggerOrder {
    pub trigger_order_bit: u8,
    pub order_price: u64,
    pub trigger_price: Option<u64>,
    pub trigger_direction: Option<TriggerDirection>,
    pub trigger_ts: Option<u64>,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub tag: Option<String>,
    pub asset: Asset,
}

pub struct PlaceTriggerOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub trigger_order: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceTriggerOrder {
    type ArrangedAccounts = PlaceTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, open_orders, authority, margin_account, pricing, trigger_order, system_program, dex_program, market, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlaceTriggerOrderInstructionAccounts {
            state: state.pubkey,
            open_orders: open_orders.pubkey,
            authority: authority.pubkey,
            margin_account: margin_account.pubkey,
            pricing: pricing.pubkey,
            trigger_order: trigger_order.pubkey,
            system_program: system_program.pubkey,
            dex_program: dex_program.pubkey,
            market: market.pubkey,
        })
    }
}
