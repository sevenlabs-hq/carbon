
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CreateMetadataAccountArgsV3 {
    pub data: DataV2,
    pub is_mutable: bool,
    pub collection_details: Option<CollectionDetails>,
}
