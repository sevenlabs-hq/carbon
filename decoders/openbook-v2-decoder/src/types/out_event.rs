use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OutEvent {
    pub event_type: u8,
    pub side: u8,
    pub owner_slot: u8,
    pub padding0: [u8; 5],
    pub timestamp: u64,
    pub seq_num: u64,
    pub owner: solana_pubkey::Pubkey,
    pub quantity: i64,
    #[serde(with = "BigArray")]
    pub padding1: [u8; 80],
}
