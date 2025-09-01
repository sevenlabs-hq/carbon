use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolLpTokenSupply {
    pub initial: u64,
    pub total: u64,
    pub unlocked: u64,
    pub locked: u64,
    pub burnt: u64,
}
