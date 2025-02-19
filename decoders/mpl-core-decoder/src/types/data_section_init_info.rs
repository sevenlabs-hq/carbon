use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DataSectionInitInfo {
    pub parent_key: LinkedDataKey,
    pub schema: ExternalPluginAdapterSchema,
}
