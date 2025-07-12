use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe90a12e681ac25ea")]
pub struct EditionMarker {
    pub key: Key,
    pub ledger: [u8; 31],
}
