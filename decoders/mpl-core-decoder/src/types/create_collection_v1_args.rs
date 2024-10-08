
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CreateCollectionV1Args {
    pub name: String,
    pub uri: String,
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}
