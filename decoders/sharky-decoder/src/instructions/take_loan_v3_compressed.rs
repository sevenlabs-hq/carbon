use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf1726a4f1059e97d")]
pub struct TakeLoanV3Compressed {
    pub expected_loan: String,
    pub nft_list_index: Option<u32>,
    pub cnft_args: CnftArgs,
}

pub struct TakeLoanV3CompressedInstructionAccounts {
    pub lender: solana_sdk::pubkey::Pubkey,
    pub borrower: solana_sdk::pubkey::Pubkey,
    pub loan: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub collateral_mint: solana_sdk::pubkey::Pubkey,
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub mpl_bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeLoanV3Compressed {
    type ArrangedAccounts = TakeLoanV3CompressedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lender, borrower, loan, escrow, order_book, collateral_mint, tree_authority, log_wrapper, merkle_tree, system_program, token_program, mpl_bubblegum_program, compression_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TakeLoanV3CompressedInstructionAccounts {
            lender: lender.pubkey,
            borrower: borrower.pubkey,
            loan: loan.pubkey,
            escrow: escrow.pubkey,
            order_book: order_book.pubkey,
            collateral_mint: collateral_mint.pubkey,
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
