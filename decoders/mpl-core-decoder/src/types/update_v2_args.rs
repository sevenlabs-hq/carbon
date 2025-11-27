use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateV2Args {
    pub new_name: Option<String>,
    pub new_uri: Option<String>,
    pub new_update_authority: Option<UpdateAuthority>,
}
