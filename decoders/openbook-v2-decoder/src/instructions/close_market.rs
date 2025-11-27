use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x589af8ba300e7bf4")]
pub struct CloseMarket {}

pub struct CloseMarketInstructionAccounts {
    pub close_market_admin: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub bids: solana_pubkey::Pubkey,
    pub asks: solana_pubkey::Pubkey,
    pub event_heap: solana_pubkey::Pubkey,
    pub sol_destination: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseMarket {
    type ArrangedAccounts = CloseMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [close_market_admin, market, bids, asks, event_heap, sol_destination, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
