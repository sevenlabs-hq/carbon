
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RegistryRecord {
    pub plugin_type: PluginType,
    pub authority: Authority,
    pub offset: u64,
}
