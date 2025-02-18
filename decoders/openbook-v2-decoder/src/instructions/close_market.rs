use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x589af8ba300e7bf4")]
pub struct CloseMarket {}

pub struct CloseMarketInstructionAccounts {
    pub close_market_admin: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
    pub sol_destination: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseMarket {
    type ArrangedAccounts = CloseMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let close_market_admin = accounts.get(0)?;
        let market = accounts.get(1)?;
        let bids = accounts.get(2)?;
        let asks = accounts.get(3)?;
        let event_heap = accounts.get(4)?;
        let sol_destination = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(CloseMarketInstructionAccounts {
            close_market_admin: close_market_admin.pubkey,
            market: market.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            event_heap: event_heap.pubkey,
            sol_destination: sol_destination.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
