use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct PluginRegistryV1 {
    pub key: Key,
    pub registry: Vec<RegistryRecord>,
    pub external_registry: Vec<ExternalRegistryRecord>,
}
