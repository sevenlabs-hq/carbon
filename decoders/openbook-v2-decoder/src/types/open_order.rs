use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OpenOrder {
    pub id: u128,
    pub client_id: u64,
    pub locked_price: i64,
    pub is_free: u8,
    pub side_and_tree: u8,
    pub padding: [u8; 6],
}
