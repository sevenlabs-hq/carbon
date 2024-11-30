
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WriteCollectionExternalPluginAdapterDataV1Args {
    pub key: ExternalPluginAdapterKey,
    pub data: Option<Vec<u8>>,
}
