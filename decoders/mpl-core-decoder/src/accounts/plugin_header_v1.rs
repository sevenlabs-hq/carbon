use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xed32a5d026fd9998")]
pub struct PluginHeaderV1 {
    pub key: Key,
    pub plugin_registry_offset: u64,
}
