 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use crate::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9eb1fe6cb8be2a0d")] 
pub struct Watch { 
}