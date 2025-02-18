use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x032c47031ac7cb55")]
pub struct PlaceTakeOrder {
    pub args: PlaceTakeOrderArgs,
}

pub struct PlaceTakeOrderInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub penalty_payer: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub market_authority: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub market_base_vault: solana_sdk::pubkey::Pubkey,
    pub market_quote_vault: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
    pub user_base_account: solana_sdk::pubkey::Pubkey,
    pub user_quote_account: solana_sdk::pubkey::Pubkey,
    pub oracle_a: solana_sdk::pubkey::Pubkey,
    pub oracle_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub open_orders_admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceTakeOrder {
    type ArrangedAccounts = PlaceTakeOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let signer = accounts.get(0)?;
        let penalty_payer = accounts.get(1)?;
        let market = accounts.get(2)?;
        let market_authority = accounts.get(3)?;
        let bids = accounts.get(4)?;
        let asks = accounts.get(5)?;
        let market_base_vault = accounts.get(6)?;
        let market_quote_vault = accounts.get(7)?;
        let event_heap = accounts.get(8)?;
        let user_base_account = accounts.get(9)?;
        let user_quote_account = accounts.get(10)?;
        let oracle_a = accounts.get(11)?;
        let oracle_b = accounts.get(12)?;
        let token_program = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let open_orders_admin = accounts.get(15)?;

        Some(PlaceTakeOrderInstructionAccounts {
            signer: signer.pubkey,
            penalty_payer: penalty_payer.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            market_base_vault: market_base_vault.pubkey,
            market_quote_vault: market_quote_vault.pubkey,
            event_heap: event_heap.pubkey,
            user_base_account: user_base_account.pubkey,
            user_quote_account: user_quote_account.pubkey,
            oracle_a: oracle_a.pubkey,
            oracle_b: oracle_b.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            open_orders_admin: open_orders_admin.pubkey,
        })
    }
}
