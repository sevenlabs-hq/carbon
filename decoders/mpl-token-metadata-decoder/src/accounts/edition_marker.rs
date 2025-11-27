use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EditionMarker {
    pub key: Key,
    pub ledger: [u8; 31],
}
