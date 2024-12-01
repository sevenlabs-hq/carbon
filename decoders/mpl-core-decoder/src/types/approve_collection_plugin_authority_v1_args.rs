
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ApproveCollectionPluginAuthorityV1Args {
    pub plugin_type: PluginType,
    pub new_authority: Authority,
}
