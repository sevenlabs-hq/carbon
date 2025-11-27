use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TakenLoan {
    pub nft_collateral_mint: solana_pubkey::Pubkey,
    pub lender_note_mint: solana_pubkey::Pubkey,
    pub borrower_note_mint: solana_pubkey::Pubkey,
    pub apy: APY,
    pub terms: LoanTerms,
    pub is_collateral_frozen: u8,
}
