use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LeafNode {
    pub tag: u8,
    pub owner_slot: u8,
    pub time_in_force: u16,
    pub padding: [u8; 4],
    pub key: u128,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub quantity: i64,
    pub timestamp: u64,
    pub peg_limit: i64,
    pub client_order_id: u64,
}
