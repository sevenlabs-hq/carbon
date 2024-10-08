 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x384b9f4c8e44be69")] 
pub struct FeeTier { 
        pub whirlpools_config: solana_sdk::pubkey::Pubkey, 
        pub tick_spacing: u16, 
        pub default_fee_rate: u16, 
}