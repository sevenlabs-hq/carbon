use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AppDataInitInfo {
    pub data_authority: Authority,
    pub init_plugin_authority: Option<Authority>,
    pub schema: Option<ExternalPluginAdapterSchema>,
}
