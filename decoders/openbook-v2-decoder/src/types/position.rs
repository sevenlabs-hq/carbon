use {
    carbon_core::{borsh, CarbonDeserialize},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Position {
    pub bids_base_lots: i64,
    pub asks_base_lots: i64,
    pub base_free_native: u64,
    pub quote_free_native: u64,
    pub locked_maker_fees: u64,
    pub referrer_rebates_available: u64,
    pub penalty_heap_count: u64,
    pub maker_volume: u128,
    pub taker_volume: u128,
    pub bids_quote_lots: i64,
    #[serde(with = "BigArray")]
    pub reserved: [u8; 64],
}
