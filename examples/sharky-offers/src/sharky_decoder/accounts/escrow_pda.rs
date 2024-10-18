
 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xa8d91e0ef8c57680")] 
pub struct EscrowPda { 
        pub bump: u8, 
}