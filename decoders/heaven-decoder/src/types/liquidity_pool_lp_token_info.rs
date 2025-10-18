use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolLpTokenInfo {
    pub supply: LiquidityPoolLpTokenSupply,
    pub decimals: u8,
    pub pad: [u8; 7],
}
