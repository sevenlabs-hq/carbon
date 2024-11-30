
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LinkedAppDataInitInfo {
    pub data_authority: Authority,
    pub init_plugin_authority: Option<Authority>,
    pub schema: Option<ExternalPluginAdapterSchema>,
}
