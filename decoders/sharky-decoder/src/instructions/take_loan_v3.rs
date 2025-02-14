use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xff73dc3a1a9d70b9")]
pub struct TakeLoanV3 {
    pub expected_loan: String,
    pub nft_list_index: Option<u32>,
    pub skip_freezing_collateral: bool,
}

pub struct TakeLoanV3InstructionAccounts {
    pub lender: solana_sdk::pubkey::Pubkey,
    pub borrower: solana_sdk::pubkey::Pubkey,
    pub borrower_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub collateral_mint: solana_sdk::pubkey::Pubkey,
    pub loan: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_collateral_token_account: solana_sdk::pubkey::Pubkey,
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub mpl_token_metadata_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeLoanV3 {
    type ArrangedAccounts = TakeLoanV3InstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lender = accounts.get(0)?;
        let borrower = accounts.get(1)?;
        let borrower_collateral_token_account = accounts.get(2)?;
        let collateral_mint = accounts.get(3)?;
        let loan = accounts.get(4)?;
        let escrow = accounts.get(5)?;
        let escrow_collateral_token_account = accounts.get(6)?;
        let order_book = accounts.get(7)?;
        let metadata = accounts.get(8)?;
        let edition = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;
        let rent = accounts.get(13)?;
        let mpl_token_metadata_program = accounts.get(14)?;

        Some(TakeLoanV3InstructionAccounts {
            lender: lender.pubkey,
            borrower: borrower.pubkey,
            borrower_collateral_token_account: borrower_collateral_token_account.pubkey,
            collateral_mint: collateral_mint.pubkey,
            loan: loan.pubkey,
            escrow: escrow.pubkey,
            escrow_collateral_token_account: escrow_collateral_token_account.pubkey,
            order_book: order_book.pubkey,
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
