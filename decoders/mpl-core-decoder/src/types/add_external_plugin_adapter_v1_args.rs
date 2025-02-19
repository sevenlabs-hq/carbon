use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddExternalPluginAdapterV1Args {
    pub init_info: ExternalPluginAdapterInitInfo,
}
