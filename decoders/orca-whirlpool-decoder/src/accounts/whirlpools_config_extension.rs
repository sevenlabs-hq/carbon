 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x0263d7a3f01a993a")] 
pub struct WhirlpoolsConfigExtension { 
        pub whirlpools_config: solana_sdk::pubkey::Pubkey, 
        pub config_extension_authority: solana_sdk::pubkey::Pubkey, 
        pub token_badge_authority: solana_sdk::pubkey::Pubkey, 
}