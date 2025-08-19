use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FIFOOrderId {
    pub price_in_ticks: Ticks,
    pub order_sequence_number: u64,
}
