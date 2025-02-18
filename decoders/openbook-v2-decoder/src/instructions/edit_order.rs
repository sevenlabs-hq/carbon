use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfed0761dadf8c846")]
pub struct EditOrder {
    pub client_order_id: u64,
    pub expected_cancel_size: i64,
    pub place_order: PlaceOrderArgs,
}

pub struct EditOrderInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub open_orders_account: solana_sdk::pubkey::Pubkey,
    pub open_orders_admin: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
    pub market_vault: solana_sdk::pubkey::Pubkey,
    pub oracle_a: solana_sdk::pubkey::Pubkey,
    pub oracle_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditOrder {
    type ArrangedAccounts = EditOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let signer = accounts.get(0)?;
        let open_orders_account = accounts.get(1)?;
        let open_orders_admin = accounts.get(2)?;
        let user_token_account = accounts.get(3)?;
        let market = accounts.get(4)?;
        let bids = accounts.get(5)?;
        let asks = accounts.get(6)?;
        let event_heap = accounts.get(7)?;
        let market_vault = accounts.get(8)?;
        let oracle_a = accounts.get(9)?;
        let oracle_b = accounts.get(10)?;
        let token_program = accounts.get(11)?;

        Some(EditOrderInstructionAccounts {
            signer: signer.pubkey,
            open_orders_account: open_orders_account.pubkey,
            open_orders_admin: open_orders_admin.pubkey,
            user_token_account: user_token_account.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_heap: event_heap.pubkey,
            market_vault: market_vault.pubkey,
            oracle_a: oracle_a.pubkey,
            oracle_b: oracle_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
