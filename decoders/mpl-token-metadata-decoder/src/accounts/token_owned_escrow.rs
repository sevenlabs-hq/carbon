 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x1589745b7b627ee4")] 
pub struct TokenOwnedEscrow { 
        pub key: Key, 
        pub base_token: solana_sdk::pubkey::Pubkey, 
        pub authority: EscrowAuthority, 
        pub bump: u8, 
}