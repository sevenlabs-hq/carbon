 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x4561bdbe6e0742bb")] 
pub struct TickArray { 
        pub start_tick_index: i32, 
        pub ticks: [Tick; 88], 
        pub whirlpool: solana_sdk::pubkey::Pubkey, 
}