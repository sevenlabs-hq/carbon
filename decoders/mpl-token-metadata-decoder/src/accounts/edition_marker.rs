use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct EditionMarker {
    pub key: Key,
    pub ledger: [u8; 31],
}
