use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x617b55364c103d9d")]
pub struct RepayLoanV3 {}

pub struct RepayLoanV3InstructionAccounts {
    pub loan: solana_sdk::pubkey::Pubkey,
    pub borrower: solana_sdk::pubkey::Pubkey,
    pub borrower_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub lender: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub value_mint: solana_sdk::pubkey::Pubkey,
    pub collateral_mint: solana_sdk::pubkey::Pubkey,
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub mpl_token_metadata_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RepayLoanV3 {
    type ArrangedAccounts = RepayLoanV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [loan, borrower, borrower_collateral_token_account, lender, escrow, escrow_collateral_token_account, value_mint, collateral_mint, order_book, fee_authority, metadata, edition, system_program, token_program, associated_token_program, rent, mpl_token_metadata_program] =
            accounts
        else {
            return None;
        };

        Some(RepayLoanV3InstructionAccounts {
            loan: loan.pubkey,
            borrower: borrower.pubkey,
            borrower_collateral_token_account: borrower_collateral_token_account.pubkey,
            lender: lender.pubkey,
            escrow: escrow.pubkey,
            escrow_collateral_token_account: escrow_collateral_token_account.pubkey,
            value_mint: value_mint.pubkey,
            collateral_mint: collateral_mint.pubkey,
            order_book: order_book.pubkey,
            fee_authority: fee_authority.pubkey,
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
            mpl_token_metadata_program: mpl_token_metadata_program.pubkey,
        })
    }
}
