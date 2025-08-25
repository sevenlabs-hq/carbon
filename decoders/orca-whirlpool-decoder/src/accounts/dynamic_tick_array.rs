use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11d8f68ee1c7da38")]
pub struct DynamicTickArray {
    pub start_tick_index: i32,
    pub whirlpool: solana_pubkey::Pubkey,
    pub tick_bitmap: u128,
    #[serde(with = "serde_big_array::BigArray")]
    pub ticks: [DynamicTick; 88],
}
