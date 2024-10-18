
 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x8a68f4c5ce2f9f9a")] 
pub struct ProgramVersion { 
        pub version: u8, 
        pub bump: u8, 
        pub updated: i64, 
}