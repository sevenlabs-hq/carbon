
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct UpdateV1Args {
    pub new_name: Option<String>,
    pub new_uri: Option<String>,
    pub new_update_authority: Option<UpdateAuthority>,
}
