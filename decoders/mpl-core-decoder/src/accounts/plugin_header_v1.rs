 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xed32a5d026fd9998")] 
pub struct PluginHeaderV1 { 
        pub key: Key, 
        pub plugin_registry_offset: u64, 
}