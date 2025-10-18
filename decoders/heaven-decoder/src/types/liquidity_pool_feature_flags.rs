use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolFeatureFlags {
    pub sandwich_resistant_mode: u8,
    pub padding1: [u8; 7],
}
