 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x605b68399123ac9b")] 
pub struct Vote { 
}