use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateCollectionExternalPluginAdapterV1Args {
    pub key: ExternalPluginAdapterKey,
    pub update_info: ExternalPluginAdapterUpdateInfo,
}
