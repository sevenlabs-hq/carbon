use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa765a1f6d03106e1")]
pub struct CloseOpenOrdersV4 {
    pub map_nonce: u8,
    pub asset: Asset,
}

pub struct CloseOpenOrdersV4InstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub cross_margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub open_orders_map: solana_pubkey::Pubkey,
    pub event_queue: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOpenOrdersV4 {
    type ArrangedAccounts = CloseOpenOrdersV4InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, pricing, dex_program, open_orders, cross_margin_account, authority, market, serum_authority, open_orders_map, event_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseOpenOrdersV4InstructionAccounts {
            state: state.pubkey,
            pricing: pricing.pubkey,
            dex_program: dex_program.pubkey,
            open_orders: open_orders.pubkey,
            cross_margin_account: cross_margin_account.pubkey,
            authority: authority.pubkey,
            market: market.pubkey,
            serum_authority: serum_authority.pubkey,
            open_orders_map: open_orders_map.pubkey,
            event_queue: event_queue.pubkey,
        })
    }
}
