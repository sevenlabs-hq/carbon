use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xed32a5d026fd9998")]
pub struct PluginHeaderV1 {
    pub key: Key,
    pub plugin_registry_offset: u64,
}
