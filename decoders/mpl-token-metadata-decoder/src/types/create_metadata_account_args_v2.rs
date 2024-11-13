
use super::*;
use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CreateMetadataAccountArgsV2 {
    pub data: DataV2,
    pub is_mutable: bool,
}

