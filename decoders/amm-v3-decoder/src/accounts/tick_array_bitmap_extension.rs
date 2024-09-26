 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use crate::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x3c9624db61808b99")] 
pub struct TickArrayBitmapExtension { 
        pub pool_id: solana_sdk::pubkey::Pubkey, 
        pub positive_tick_array_bitmap: [[u64; 8]; 14], 
        pub negative_tick_array_bitmap: [[u64; 8]; 14], 
}