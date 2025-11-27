use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ReduceEvent {
    pub index: u16,
    pub order_sequence_number: u64,
    pub price_in_ticks: u64,
    pub base_lots_removed: u64,
    pub base_lots_remaining: u64,
}
