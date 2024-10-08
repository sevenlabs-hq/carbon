 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xe3c8e6c5f4c6ac32")] 
pub struct UseAuthorityRecord { 
        pub key: Key, 
        pub allowed_uses: u64, 
        pub bump: u8, 
}