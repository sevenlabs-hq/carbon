use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x94a14b578a22833e")]
pub struct ExtendLoanV3Compressed {
    pub expected_loan: String,
    pub cnft_root: [u8; 32],
    pub cnft_data_hash: [u8; 32],
    pub cnft_creator_hash: [u8; 32],
    pub cnft_nonce: u64,
    pub cnft_index: u32,
}

pub struct ExtendLoanV3CompressedInstructionAccounts {
    pub loan: solana_sdk::pubkey::Pubkey,
    pub new_loan: solana_sdk::pubkey::Pubkey,
    pub borrower: solana_sdk::pubkey::Pubkey,
    pub lender: solana_sdk::pubkey::Pubkey,
    pub new_lender: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub new_escrow: solana_sdk::pubkey::Pubkey,
    pub value_mint: solana_sdk::pubkey::Pubkey,
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub mpl_bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExtendLoanV3Compressed {
    type ArrangedAccounts = ExtendLoanV3CompressedInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let loan = accounts.get(0)?;
        let new_loan = accounts.get(1)?;
        let borrower = accounts.get(2)?;
        let lender = accounts.get(3)?;
        let new_lender = accounts.get(4)?;
        let escrow = accounts.get(5)?;
        let new_escrow = accounts.get(6)?;
        let value_mint = accounts.get(7)?;
        let order_book = accounts.get(8)?;
        let fee_authority = accounts.get(9)?;
        let tree_authority = accounts.get(10)?;
        let log_wrapper = accounts.get(11)?;
        let merkle_tree = accounts.get(12)?;
        let system_program = accounts.get(13)?;
        let token_program = accounts.get(14)?;
        let mpl_bubblegum_program = accounts.get(15)?;
        let compression_program = accounts.get(16)?;
        let rent = accounts.get(17)?;

        Some(ExtendLoanV3CompressedInstructionAccounts {
            loan: loan.pubkey,
            new_loan: new_loan.pubkey,
            borrower: borrower.pubkey,
            lender: lender.pubkey,
            new_lender: new_lender.pubkey,
            escrow: escrow.pubkey,
            new_escrow: new_escrow.pubkey,
            value_mint: value_mint.pubkey,
            order_book: order_book.pubkey,
            fee_authority: fee_authority.pubkey,
            tree_authority: tree_authority.pubkey,
            log_wrapper: log_wrapper.pubkey,
            merkle_tree: merkle_tree.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            mpl_bubblegum_program: mpl_bubblegum_program.pubkey,
            compression_program: compression_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
