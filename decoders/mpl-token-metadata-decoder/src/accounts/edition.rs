 
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xea75f94a0763eba7")] 
pub struct Edition { 
        pub key: Key, 
        pub parent: solana_sdk::pubkey::Pubkey, 
        pub edition: u64, 
}
