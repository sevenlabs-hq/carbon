use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BuyEvent {
    pub pool: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub new_reserves_a: u128,
    pub new_reserves_b: u128,
    pub fee_a: u64,
}
