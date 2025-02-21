use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateGreeksArgs {
    pub index: u8,
    pub theo: u64,
    pub delta: u32,
    pub gamma: u32,
    pub volatility: u32,
}
