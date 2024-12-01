use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct EditionMarkerV2 {
    pub key: Key,
    pub ledger: Vec<u8>,
}
