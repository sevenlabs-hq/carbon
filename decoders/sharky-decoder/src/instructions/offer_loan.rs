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
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lender = accounts.get(0)?;
        let lender_value_token_account = accounts.get(1)?;
        let value_mint = accounts.get(2)?;
        let loan = accounts.get(3)?;
        let escrow = accounts.get(4)?;
        let escrow_token_account = accounts.get(5)?;
        let order_book = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let rent = accounts.get(10)?;

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
