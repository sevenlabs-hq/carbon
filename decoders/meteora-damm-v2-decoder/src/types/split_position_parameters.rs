use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SplitPositionParameters {
    /// Percentage of unlocked liquidity to split to the second position
    pub unlocked_liquidity_percentage: u8,
    /// Percentage of permanent locked liquidity to split to the second position
    pub permanent_locked_liquidity_percentage: u8,
    /// Percentage of fee A pending to split to the second position
    pub fee_a_percentage: u8,
    /// Percentage of fee B pending to split to the second position
    pub fee_b_percentage: u8,
    /// Percentage of reward 0 pending to split to the second position
    pub reward0_percentage: u8,
    /// Percentage of reward 1 pending to split to the second position
    pub reward1_percentage: u8,
    /// padding for future
    pub padding: [u8; 16],
}
