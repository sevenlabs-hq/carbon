

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct UpdateCollectionV1Args {
    pub new_name: Option<String>,
    pub new_uri: Option<String>,
}
