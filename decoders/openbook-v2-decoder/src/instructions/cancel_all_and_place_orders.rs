use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x809bde3cba28e132")]
pub struct CancelAllAndPlaceOrders {
    pub orders_type: PlaceOrderType,
    pub bids: Vec<PlaceMultipleOrdersArgs>,
    pub asks: Vec<PlaceMultipleOrdersArgs>,
    pub limit: u8,
}

pub struct CancelAllAndPlaceOrdersInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub open_orders_admin: solana_sdk::pubkey::Pubkey,
    pub user_quote_account: solana_sdk::pubkey::Pubkey,
    pub user_base_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
    pub market_quote_vault: solana_sdk::pubkey::Pubkey,
    pub market_base_vault: solana_sdk::pubkey::Pubkey,
    pub oracle_a: solana_sdk::pubkey::Pubkey,
    pub oracle_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelAllAndPlaceOrders {
    type ArrangedAccounts = CancelAllAndPlaceOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let signer = accounts.get(0)?;
        let open_orders_account = accounts.get(1)?;
        let open_orders_admin = accounts.get(2)?;
        let user_quote_account = accounts.get(3)?;
        let user_base_account = accounts.get(4)?;
        let market = accounts.get(5)?;
        let bids = accounts.get(6)?;
        let asks = accounts.get(7)?;
        let event_heap = accounts.get(8)?;
        let market_quote_vault = accounts.get(9)?;
        let market_base_vault = accounts.get(10)?;
        let oracle_a = accounts.get(11)?;
        let oracle_b = accounts.get(12)?;
        let token_program = accounts.get(13)?;

        Some(CancelAllAndPlaceOrdersInstructionAccounts {
            signer: signer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            open_orders_admin: open_orders_admin.pubkey,
            user_quote_account: user_quote_account.pubkey,
            user_base_account: user_base_account.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_heap: event_heap.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            market_base_vault: market_base_vault.pubkey,
            oracle_a: oracle_a.pubkey,
            oracle_b: oracle_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
