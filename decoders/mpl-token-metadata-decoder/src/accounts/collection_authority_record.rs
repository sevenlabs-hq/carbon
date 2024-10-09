 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9c306c1fd4db64a8")] 
pub struct CollectionAuthorityRecord { 
        pub key: Key, 
        pub bump: u8, 
        pub update_authority: Option<solana_sdk::pubkey::Pubkey>, 
}