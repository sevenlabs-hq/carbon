use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x5c37389de6b8ab42")]
pub struct PerpSyncQueue {
    pub nonce: u8,
    pub head: u16,
    pub length: u16,
    pub queue: [AnchorDecimal; 600],
}
