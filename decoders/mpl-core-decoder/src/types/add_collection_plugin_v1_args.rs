
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct AddCollectionPluginV1Args {
    pub plugin: Plugin,
    pub init_authority: Option<Authority>,
}
