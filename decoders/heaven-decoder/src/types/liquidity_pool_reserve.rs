use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolReserve {
    pub token_a: u64,
    pub token_b: u64,
    pub snapshot_slot: u64,
    pub snapshot_a: u64,
    pub snapshot_b: u64,
    pub padding: u64,
    pub initial_a: u64,
    pub initial_b: u64,
    pub leader_slot_window: u8,
    pub pad: [u8; 7],
}
