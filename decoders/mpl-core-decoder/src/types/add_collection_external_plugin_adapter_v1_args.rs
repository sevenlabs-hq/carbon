
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct AddCollectionExternalPluginAdapterV1Args {
    pub init_info: ExternalPluginAdapterInitInfo,
}
