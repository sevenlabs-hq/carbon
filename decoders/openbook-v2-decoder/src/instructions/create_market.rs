use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x67e261ebc8bcfbfe")]
pub struct CreateMarket {
    pub name: String,
    pub oracle_config: OracleConfigParams,
    pub quote_lot_size: i64,
    pub base_lot_size: i64,
    pub maker_fee: i64,
    pub taker_fee: i64,
    pub time_expiry: i64,
}

pub struct CreateMarketInstructionAccounts {
    pub market: solana_sdk::pubkey::Pubkey,
    pub market_authority: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub market_base_vault: solana_sdk::pubkey::Pubkey,
    pub market_quote_vault: solana_sdk::pubkey::Pubkey,
    pub base_mint: solana_sdk::pubkey::Pubkey,
    pub quote_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub oracle_a: solana_sdk::pubkey::Pubkey,
    pub oracle_b: solana_sdk::pubkey::Pubkey,
    pub collect_fee_admin: solana_sdk::pubkey::Pubkey,
    pub open_orders_admin: solana_sdk::pubkey::Pubkey,
    pub consume_events_admin: solana_sdk::pubkey::Pubkey,
    pub close_market_admin: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMarket {
    type ArrangedAccounts = CreateMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let market = accounts.get(0)?;
        let market_authority = accounts.get(1)?;
        let bids = accounts.get(2)?;
        let asks = accounts.get(3)?;
        let event_heap = accounts.get(4)?;
        let payer = accounts.get(5)?;
        let market_base_vault = accounts.get(6)?;
        let market_quote_vault = accounts.get(7)?;
        let base_mint = accounts.get(8)?;
        let quote_mint = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;
        let oracle_a = accounts.get(13)?;
        let oracle_b = accounts.get(14)?;
        let collect_fee_admin = accounts.get(15)?;
        let open_orders_admin = accounts.get(16)?;
        let consume_events_admin = accounts.get(17)?;
        let close_market_admin = accounts.get(18)?;
        let event_authority = accounts.get(19)?;
        let program = accounts.get(20)?;

        Some(CreateMarketInstructionAccounts {
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_heap: event_heap.pubkey,
            payer: payer.pubkey,
            market_base_vault: market_base_vault.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            base_mint: base_mint.pubkey,
            quote_mint: quote_mint.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            oracle_a: oracle_a.pubkey,
            oracle_b: oracle_b.pubkey,
            collect_fee_admin: collect_fee_admin.pubkey,
            open_orders_admin: open_orders_admin.pubkey,
            consume_events_admin: consume_events_admin.pubkey,
            close_market_admin: close_market_admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
