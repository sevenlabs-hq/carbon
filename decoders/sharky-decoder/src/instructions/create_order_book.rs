use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x997209336444f0c5")]
pub struct CreateOrderBook {
    pub order_book_type: OrderBookType,
    pub apy: APY,
    pub loan_terms: BookLoanTerms,
    pub fee_permillicentage: u16,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
}

pub struct CreateOrderBookInstructionAccounts {
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrderBook {
    type ArrangedAccounts = CreateOrderBookInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let order_book = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(CreateOrderBookInstructionAccounts {
            order_book: order_book.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
