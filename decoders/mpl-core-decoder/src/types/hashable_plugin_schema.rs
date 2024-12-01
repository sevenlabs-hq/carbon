
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct HashablePluginSchema {
    pub index: u64,
    pub authority: Authority,
    pub plugin: Plugin,
}
