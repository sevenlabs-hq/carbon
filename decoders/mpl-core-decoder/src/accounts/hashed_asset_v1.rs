 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xc56d2e767fef7e32")] 
pub struct HashedAssetV1 { 
        pub key: Key, 
        pub hash: [u8; 32], 
}