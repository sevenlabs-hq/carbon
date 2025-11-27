use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolTokenInfo {
    pub mint: solana_pubkey::Pubkey,
    pub decimals: u8,
    pub owner: solana_pubkey::Pubkey,
}
