
use super::*;
use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CreateMetadataAccountArgs {
    pub data: Data,
    pub is_mutable: bool,
}

