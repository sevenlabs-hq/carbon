use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FeeEvent {
    pub index: u16,
    pub fees_collected_in_quote_lots: u64,
}
