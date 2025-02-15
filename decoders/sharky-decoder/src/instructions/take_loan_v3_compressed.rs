use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

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
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lender = accounts.get(0)?;
        let borrower = accounts.get(1)?;
        let loan = accounts.get(2)?;
        let escrow = accounts.get(3)?;
        let order_book = accounts.get(4)?;
        let collateral_mint = accounts.get(5)?;
        let tree_authority = accounts.get(6)?;
        let log_wrapper = accounts.get(7)?;
        let merkle_tree = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let mpl_bubblegum_program = accounts.get(11)?;
        let compression_program = accounts.get(12)?;
        let rent = accounts.get(13)?;

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
