use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FillSummaryEvent {
    pub index: u16,
    pub client_order_id: u128,
    pub total_base_lots_filled: u64,
    pub total_quote_lots_filled: u64,
    pub total_fee_in_quote_lots: u64,
}
