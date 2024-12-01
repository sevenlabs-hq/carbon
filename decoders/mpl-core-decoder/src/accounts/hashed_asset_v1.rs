use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct HashedAssetV1 {
    pub key: Key,
    pub hash: [u8; 32],
}
