use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbddb7fd34ee661ee")]
pub struct TradeEvent {
    pub mint: solana_pubkey::Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: solana_pubkey::Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub fee_basis_points: u64,
    pub fee: u64,
    pub creator: solana_pubkey::Pubkey,
    pub creator_fee_basis_points: u64,
    pub creator_fee: u64,
}
