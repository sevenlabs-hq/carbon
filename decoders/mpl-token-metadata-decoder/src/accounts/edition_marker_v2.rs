 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x837b3cfb2d02546e")] 
pub struct EditionMarkerV2 { 
        pub key: Key, 
        pub ledger: Vec<u8>, 
}