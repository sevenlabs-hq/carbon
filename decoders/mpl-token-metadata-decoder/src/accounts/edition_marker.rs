use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
pub struct EditionMarker {
    pub key: Key,
    pub ledger: [u8; 31],
}
