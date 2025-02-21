use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlaceEvent {
    pub index: u16,
    pub order_sequence_number: u64,
    pub client_order_id: u128,
    pub price_in_ticks: u64,
    pub base_lots_placed: u64,
}
