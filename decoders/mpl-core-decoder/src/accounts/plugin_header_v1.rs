use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct PluginHeaderV1 {
    pub key: Key,
    pub plugin_registry_offset: u64,
}
