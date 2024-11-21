use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x871aa32bc6dd1d43")]
pub struct OpenBookV2Swap {}

pub struct OpenBookV2SwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
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

impl carbon_core::deserialize::ArrangeAccounts for OpenBookV2Swap {
    type ArrangedAccounts = OpenBookV2SwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let signer = accounts.get(1)?;
        let penalty_payer = accounts.get(2)?;
        let market = accounts.get(3)?;
        let market_authority = accounts.get(4)?;
        let bids = accounts.get(5)?;
        let asks = accounts.get(6)?;
        let market_base_vault = accounts.get(7)?;
        let market_quote_vault = accounts.get(8)?;
        let event_heap = accounts.get(9)?;
        let user_base_account = accounts.get(10)?;
        let user_quote_account = accounts.get(11)?;
        let oracle_a = accounts.get(12)?;
        let oracle_b = accounts.get(13)?;
        let token_program = accounts.get(14)?;
        let system_program = accounts.get(15)?;
        let open_orders_admin = accounts.get(16)?;

        Some(OpenBookV2SwapInstructionAccounts {
            swap_program: swap_program.pubkey,
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
