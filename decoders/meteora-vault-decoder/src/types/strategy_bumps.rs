use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StrategyBumps {
    pub strategy_index: u8,
    pub other_bumps: [u8; 10],
}
