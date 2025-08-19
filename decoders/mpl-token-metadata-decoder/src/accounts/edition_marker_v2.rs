use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x837b3cfb2d02546e")]
pub struct EditionMarkerV2 {
    pub key: Key,
    pub ledger: Vec<u8>,
}
