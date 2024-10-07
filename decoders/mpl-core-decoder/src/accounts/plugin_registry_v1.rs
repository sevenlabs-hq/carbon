 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xa916f6dce5e5a4cc")] 
pub struct PluginRegistryV1 { 
        pub key: Key, 
        pub registry: Vec<RegistryRecord>, 
        pub external_plugins: Vec<ExternalPluginRecord>, 
}