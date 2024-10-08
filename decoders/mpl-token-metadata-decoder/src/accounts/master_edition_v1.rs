 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x4fa529a7b4bf8db9")] 
pub struct MasterEditionV1 { 
        pub key: Key, 
        pub supply: u64, 
        pub max_supply: Option<u64>, 
        pub printing_mint: solana_sdk::pubkey::Pubkey, 
        pub one_time_printing_authorization_mint: solana_sdk::pubkey::Pubkey, 
}