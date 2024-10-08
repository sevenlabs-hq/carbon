 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xf23ef422b5703aaa")] 
pub struct PresetParameter { 
        pub bin_step: u16, 
        pub base_factor: u16, 
        pub filter_period: u16, 
        pub decay_period: u16, 
        pub reduction_factor: u16, 
        pub variable_fee_control: u32, 
        pub max_volatility_accumulator: u32, 
        pub min_bin_id: i32, 
        pub max_bin_id: i32, 
        pub protocol_share: u16, 
}