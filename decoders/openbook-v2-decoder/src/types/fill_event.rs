use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FillEvent {
    pub event_type: u8,
    pub taker_side: u8,
    pub maker_out: u8,
    pub maker_slot: u8,
    pub padding: [u8; 4],
    pub timestamp: u64,
    pub seq_num: u64,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub maker_timestamp: u64,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub taker_client_order_id: u64,
    pub price: i64,
    pub peg_limit: i64,
    pub quantity: i64,
    pub maker_client_order_id: u64,
    pub reserved: [u8; 8],
}
