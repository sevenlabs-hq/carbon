use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddCollectionExternalPluginAdapterV1Args {
    pub init_info: ExternalPluginAdapterInitInfo,
}
