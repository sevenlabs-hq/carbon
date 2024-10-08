
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct HashedAssetSchema {
    pub asset_hash: [u8; 32],
    pub plugin_hashes: Vec<[u8; 32]>,
}
