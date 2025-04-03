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
    pub loan: solana_pubkey::Pubkey,
    pub new_loan: solana_pubkey::Pubkey,
    pub borrower: solana_pubkey::Pubkey,
    pub lender: solana_pubkey::Pubkey,
    pub new_lender: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub new_escrow: solana_pubkey::Pubkey,
    pub value_mint: solana_pubkey::Pubkey,
    pub order_book: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub tree_authority: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
    pub merkle_tree: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub mpl_bubblegum_program: solana_pubkey::Pubkey,
    pub compression_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExtendLoanV3Compressed {
    type ArrangedAccounts = ExtendLoanV3CompressedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [loan, new_loan, borrower, lender, new_lender, escrow, new_escrow, value_mint, order_book, fee_authority, tree_authority, log_wrapper, merkle_tree, system_program, token_program, mpl_bubblegum_program, compression_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
