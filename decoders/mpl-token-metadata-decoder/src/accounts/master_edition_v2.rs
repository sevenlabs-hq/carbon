use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x653ba3cfee10aa9f")]
pub struct MasterEditionV2 {
    pub key: Key,
    pub supply: u64,
    pub max_supply: Option<u64>,
}
