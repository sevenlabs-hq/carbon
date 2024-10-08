 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x81a9af41b95f2064")] 
pub struct PositionBundle { 
        pub position_bundle_mint: solana_sdk::pubkey::Pubkey, 
        pub position_bitmap: [u8; 32], 
}