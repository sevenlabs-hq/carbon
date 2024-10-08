 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xe011749de6d41eda")] 
pub struct AssetV1 { 
        pub key: Key, 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub update_authority: UpdateAuthority, 
        pub name: String, 
        pub uri: String, 
        pub seq: Option<u64>, 
}