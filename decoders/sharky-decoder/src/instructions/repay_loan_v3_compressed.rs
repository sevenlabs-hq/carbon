use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9f9ff5a8bf9a6406")]
pub struct RepayLoanV3Compressed {
    pub cnft_root: [u8; 32],
    pub cnft_data_hash: [u8; 32],
    pub cnft_creator_hash: [u8; 32],
    pub cnft_nonce: u64,
    pub cnft_index: u32,
}

pub struct RepayLoanV3CompressedInstructionAccounts {
    pub loan: solana_sdk::pubkey::Pubkey,
    pub borrower: solana_sdk::pubkey::Pubkey,
    pub lender: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
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

impl carbon_core::deserialize::ArrangeAccounts for RepayLoanV3Compressed {
    type ArrangedAccounts = RepayLoanV3CompressedInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let loan = accounts.get(0)?;
        let borrower = accounts.get(1)?;
        let lender = accounts.get(2)?;
        let escrow = accounts.get(3)?;
        let value_mint = accounts.get(4)?;
        let order_book = accounts.get(5)?;
        let fee_authority = accounts.get(6)?;
        let tree_authority = accounts.get(7)?;
        let log_wrapper = accounts.get(8)?;
        let merkle_tree = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let mpl_bubblegum_program = accounts.get(12)?;
        let compression_program = accounts.get(13)?;
        let rent = accounts.get(14)?;

        Some(RepayLoanV3CompressedInstructionAccounts {
            loan: loan.pubkey,
            borrower: borrower.pubkey,
            lender: lender.pubkey,
            escrow: escrow.pubkey,
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
