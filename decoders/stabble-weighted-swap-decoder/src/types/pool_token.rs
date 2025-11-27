use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolToken {
    pub mint: solana_pubkey::Pubkey,
    pub decimals: u8,
    pub scaling_up: bool,
    pub scaling_factor: u64,
    pub balance: u64,
    pub weight: u64,
}
