use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x37e67dda952741f8")]
pub struct OrderBook {
    pub version: u8,
    pub order_book_type: OrderBookType,
    pub apy: APY,
    pub loan_terms: BookLoanTerms,
    pub fee_permillicentage: u16,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
}
