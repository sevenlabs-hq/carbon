use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x471b11834e493e5c")]
pub struct ExtendLoanV3 {
    pub expected_loan: String,
}

pub struct ExtendLoanV3InstructionAccounts {
    pub loan: solana_pubkey::Pubkey,
    pub new_loan: solana_pubkey::Pubkey,
    pub borrower: solana_pubkey::Pubkey,
    pub borrower_collateral_token_account: solana_pubkey::Pubkey,
    pub lender: solana_pubkey::Pubkey,
    pub new_lender: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_collateral_token_account: solana_pubkey::Pubkey,
    pub new_escrow: solana_pubkey::Pubkey,
    pub new_escrow_collateral_token_account: solana_pubkey::Pubkey,
    pub value_mint: solana_pubkey::Pubkey,
    pub collateral_mint: solana_pubkey::Pubkey,
    pub order_book: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub mpl_token_metadata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExtendLoanV3 {
    type ArrangedAccounts = ExtendLoanV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [loan, new_loan, borrower, borrower_collateral_token_account, lender, new_lender, escrow, escrow_collateral_token_account, new_escrow, new_escrow_collateral_token_account, value_mint, collateral_mint, order_book, fee_authority, metadata, edition, system_program, token_program, associated_token_program, rent, mpl_token_metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ExtendLoanV3InstructionAccounts {
            loan: loan.pubkey,
            new_loan: new_loan.pubkey,
            borrower: borrower.pubkey,
            borrower_collateral_token_account: borrower_collateral_token_account.pubkey,
            lender: lender.pubkey,
            new_lender: new_lender.pubkey,
            escrow: escrow.pubkey,
            escrow_collateral_token_account: escrow_collateral_token_account.pubkey,
            new_escrow: new_escrow.pubkey,
            new_escrow_collateral_token_account: new_escrow_collateral_token_account.pubkey,
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
