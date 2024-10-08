 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x74dbcce5f974ff96")] 
pub struct TokenBadge { 
        pub whirlpools_config: solana_sdk::pubkey::Pubkey, 
        pub token_mint: solana_sdk::pubkey::Pubkey, 
}