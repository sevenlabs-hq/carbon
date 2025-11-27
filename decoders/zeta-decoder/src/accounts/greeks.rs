use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf7d5aa9a2bf392fe")]
pub struct Greeks {
    pub nonce: u8,
    #[serde(with = "serde_big_array::BigArray")]
    pub mark_prices: [u64; 46],
    #[serde(with = "serde_big_array::BigArray")]
    pub mark_prices_padding: [u64; 91],
    pub perp_mark_price: u64,
    pub product_greeks: [ProductGreeks; 22],
    #[serde(with = "serde_big_array::BigArray")]
    pub product_greeks_padding: [ProductGreeks; 44],
    pub update_timestamp: [u64; 2],
    pub update_timestamp_padding: [u64; 4],
    pub retreat_expiration_timestamp: [u64; 2],
    pub retreat_expiration_timestamp_padding: [u64; 4],
    pub interest_rate: [i64; 2],
    pub interest_rate_padding: [i64; 4],
    pub nodes: [u64; 5],
    pub volatility: [u64; 10],
    pub volatility_padding: [u64; 20],
    #[serde(with = "serde_big_array::BigArray")]
    pub node_keys: [solana_pubkey::Pubkey; 138],
    pub halt_force_pricing: [bool; 6],
    pub perp_update_timestamp: u64,
    pub perp_funding_delta: AnchorDecimal,
    pub perp_latest_funding_rate: AnchorDecimal,
    pub perp_latest_midpoint: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 1593],
}
