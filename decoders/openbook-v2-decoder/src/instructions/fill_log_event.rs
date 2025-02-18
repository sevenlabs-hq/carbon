use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9617299498a2d740")]
pub struct FillLogEvent {
    pub market: solana_sdk::pubkey::Pubkey,
    pub taker_side: u8,
    pub maker_slot: u8,
    pub maker_out: bool,
    pub timestamp: u64,
    pub seq_num: u64,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub maker_client_order_id: u64,
    pub maker_fee: u64,
    pub maker_timestamp: u64,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub taker_client_order_id: u64,
    pub taker_fee_ceil: u64,
    pub price: i64,
    pub quantity: i64,
}
