use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xc09b55cd31f9812a")]
pub struct TickArrayState {
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub start_tick_index: i32,
    pub ticks: [TickState; 60],
    pub initialized_tick_count: u8,
    pub recent_epoch: u64,
    pub padding: [u8; 107],
}
