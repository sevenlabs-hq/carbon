use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xa916f6dce5e5a4cc")]
pub struct PluginRegistryV1 {
    pub key: Key,
    pub registry: Vec<RegistryRecord>,
    pub external_registry: Vec<ExternalRegistryRecord>,
}
