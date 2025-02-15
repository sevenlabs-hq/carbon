use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x471b11834e493e5c")]
pub struct ExtendLoanV3 {
    pub expected_loan: String,
}

pub struct ExtendLoanV3InstructionAccounts {
    pub loan: solana_sdk::pubkey::Pubkey,
    pub new_loan: solana_sdk::pubkey::Pubkey,
    pub borrower: solana_sdk::pubkey::Pubkey,
    pub borrower_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub lender: solana_sdk::pubkey::Pubkey,
    pub new_lender: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub new_escrow: solana_sdk::pubkey::Pubkey,
    pub new_escrow_collateral_token_account: solana_sdk::pubkey::Pubkey,
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

impl carbon_core::deserialize::ArrangeAccounts for ExtendLoanV3 {
    type ArrangedAccounts = ExtendLoanV3InstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let loan = accounts.get(0)?;
        let new_loan = accounts.get(1)?;
        let borrower = accounts.get(2)?;
        let borrower_collateral_token_account = accounts.get(3)?;
        let lender = accounts.get(4)?;
        let new_lender = accounts.get(5)?;
        let escrow = accounts.get(6)?;
        let escrow_collateral_token_account = accounts.get(7)?;
        let new_escrow = accounts.get(8)?;
        let new_escrow_collateral_token_account = accounts.get(9)?;
        let value_mint = accounts.get(10)?;
        let collateral_mint = accounts.get(11)?;
        let order_book = accounts.get(12)?;
        let fee_authority = accounts.get(13)?;
        let metadata = accounts.get(14)?;
        let edition = accounts.get(15)?;
        let system_program = accounts.get(16)?;
        let token_program = accounts.get(17)?;
        let associated_token_program = accounts.get(18)?;
        let rent = accounts.get(19)?;
        let mpl_token_metadata_program = accounts.get(20)?;

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
