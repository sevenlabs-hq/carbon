use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x997209336444f0c5")]
pub struct CreateOrderBook {
    pub order_book_type: OrderBookType,
    pub apy: APY,
    pub loan_terms: BookLoanTerms,
    pub fee_permillicentage: u16,
    pub fee_authority: solana_pubkey::Pubkey,
}

pub struct CreateOrderBookInstructionAccounts {
    pub order_book: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrderBook {
    type ArrangedAccounts = CreateOrderBookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [order_book, payer, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateOrderBookInstructionAccounts {
            order_book: order_book.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
