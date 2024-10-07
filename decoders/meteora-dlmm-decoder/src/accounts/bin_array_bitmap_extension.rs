 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x506f7c7137ed1205")] 
pub struct BinArrayBitmapExtension { 
        pub lb_pair: solana_sdk::pubkey::Pubkey, 
        pub positive_bin_array_bitmap: [[u64; 8]; 12], 
        pub negative_bin_array_bitmap: [[u64; 8]; 12], 
}