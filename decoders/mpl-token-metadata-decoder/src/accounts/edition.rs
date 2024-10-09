 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xea75f94a0763eba7")] 
pub struct Edition { 
        pub key: Key, 
        pub parent: solana_sdk::pubkey::Pubkey, 
        pub edition: u64, 
}