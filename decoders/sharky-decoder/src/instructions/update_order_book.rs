use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1f489fe8dc995a6d")]
pub struct UpdateOrderBook {
    pub order_book_type: Option<OrderBookType>,
    pub apy: Option<APY>,
    pub loan_terms: Option<BookLoanTerms>,
    pub fee_permillicentage: Option<u16>,
    pub fee_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateOrderBookInstructionAccounts {
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateOrderBook {
    type ArrangedAccounts = UpdateOrderBookInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let order_book = accounts.get(0)?;
        let payer = accounts.get(1)?;

        Some(UpdateOrderBookInstructionAccounts {
            order_book: order_book.pubkey,
            payer: payer.pubkey,
        })
    }
}
