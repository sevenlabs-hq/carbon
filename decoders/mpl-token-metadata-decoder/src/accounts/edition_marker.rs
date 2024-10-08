 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xe90a12e681ac25ea")] 
pub struct EditionMarker { 
        pub key: Key, 
        pub ledger: [u8; 31], 
}