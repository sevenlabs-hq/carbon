use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5c37389de6b8ab42")]
pub struct PerpSyncQueue {
    pub nonce: u8,
    pub head: u16,
    pub length: u16,
    #[serde(with = "serde_big_array::BigArray")]
    pub queue: [AnchorDecimal; 600],
}
