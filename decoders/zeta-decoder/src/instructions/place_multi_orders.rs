use {
    super::super::types::*,
    alloc::vec::Vec,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xccd7f3f33beae179")]
pub struct PlaceMultiOrders {
    pub asset: Asset,
    pub bid_orders: Vec<OrderArgs>,
    pub ask_orders: Vec<OrderArgs>,
    pub order_type: OrderType,
}

pub struct PlaceMultiOrdersInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub request_queue: solana_pubkey::Pubkey,
    pub event_queue: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub market_base_vault: solana_pubkey::Pubkey,
    pub market_quote_vault: solana_pubkey::Pubkey,
    pub zeta_base_vault: solana_pubkey::Pubkey,
    pub zeta_quote_vault: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
    pub market_base_mint: solana_pubkey::Pubkey,
    pub market_quote_mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub perp_sync_queue: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceMultiOrders {
    type ArrangedAccounts = PlaceMultiOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, state, pricing, margin_account, dex_program, token_program, serum_authority, open_orders, rent, market, request_queue, event_queue, bids, asks, market_base_vault, market_quote_vault, zeta_base_vault, zeta_quote_vault, oracle, oracle_backup_feed, oracle_backup_program, market_base_mint, market_quote_mint, mint_authority, perp_sync_queue, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlaceMultiOrdersInstructionAccounts {
            authority: authority.pubkey,
            state: state.pubkey,
            pricing: pricing.pubkey,
            margin_account: margin_account.pubkey,
            dex_program: dex_program.pubkey,
            token_program: token_program.pubkey,
            serum_authority: serum_authority.pubkey,
            open_orders: open_orders.pubkey,
            rent: rent.pubkey,
            market: market.pubkey,
            request_queue: request_queue.pubkey,
            event_queue: event_queue.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            market_base_vault: market_base_vault.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            zeta_base_vault: zeta_base_vault.pubkey,
            zeta_quote_vault: zeta_quote_vault.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
            market_base_mint: market_base_mint.pubkey,
            market_quote_mint: market_quote_mint.pubkey,
            mint_authority: mint_authority.pubkey,
            perp_sync_queue: perp_sync_queue.pubkey,
        })
    }
}
