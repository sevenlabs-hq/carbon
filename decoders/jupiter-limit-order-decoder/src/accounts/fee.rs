 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x183796faa81b65b2")] 
pub struct Fee { 
        pub maker_fee: u64, 
        pub maker_stable_fee: u64, 
        pub taker_fee: u64, 
        pub taker_stable_fee: u64, 
}