 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x653ba3cfee10aa9f")] 
pub struct MasterEditionV2 { 
        pub key: Key, 
        pub supply: u64, 
        pub max_supply: Option<u64>, 
}