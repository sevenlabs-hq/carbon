
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct DataSectionInitInfo {
    pub parent_key: LinkedDataKey,
    pub schema: ExternalPluginAdapterSchema,
}
