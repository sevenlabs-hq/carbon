use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LastUpdate {
    pub slot: u64,
    pub stale: u8,
    pub price_status: u8,
    pub placeholder: [u8; 6],
}
