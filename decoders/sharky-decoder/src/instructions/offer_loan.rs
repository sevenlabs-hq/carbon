use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2c0c4c90d2d0ef55")]
pub struct OfferLoan {
    pub escrow_bump: u8,
    pub principal_lamports: u64,
    pub terms_choice: Option<LoanTermsSpec>,
}

pub struct OfferLoanInstructionAccounts {
    pub lender: solana_sdk::pubkey::Pubkey,
    pub lender_value_token_account: solana_sdk::pubkey::Pubkey,
    pub value_mint: solana_sdk::pubkey::Pubkey,
    pub loan: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_token_account: solana_sdk::pubkey::Pubkey,
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OfferLoan {
    type ArrangedAccounts = OfferLoanInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lender, lender_value_token_account, value_mint, loan, escrow, escrow_token_account, order_book, system_program, token_program, associated_token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(OfferLoanInstructionAccounts {
            lender: lender.pubkey,
            lender_value_token_account: lender_value_token_account.pubkey,
            value_mint: value_mint.pubkey,
            loan: loan.pubkey,
            escrow: escrow.pubkey,
            escrow_token_account: escrow_token_account.pubkey,
            order_book: order_book.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
