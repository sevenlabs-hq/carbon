 
use carbon_core::{borsh, CarbonDeserialize};
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xb994256b776af3ec")] 
pub struct MetadataDelegateRecord { 
        pub key: Key, 
        pub bump: u8, 
        pub mint: solana_sdk::pubkey::Pubkey, 
        pub delegate: solana_sdk::pubkey::Pubkey, 
        pub update_authority: solana_sdk::pubkey::Pubkey, 
}
