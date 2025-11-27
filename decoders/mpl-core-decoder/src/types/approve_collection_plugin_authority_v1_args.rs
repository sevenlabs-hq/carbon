use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ApproveCollectionPluginAuthorityV1Args {
    pub plugin_type: PluginType,
    pub new_authority: Authority,
}
