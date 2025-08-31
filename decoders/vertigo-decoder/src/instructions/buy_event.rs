use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d67f4521f2cf57777")]
pub struct BuyEvent {
    pub pool: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub new_reserves_a: u128,
    pub new_reserves_b: u128,
    pub fee_a: u64,
}
