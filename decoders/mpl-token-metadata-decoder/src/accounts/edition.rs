use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xea75f94a0763eba7")]
pub struct Edition {
    pub key: Key,
    pub parent: solana_pubkey::Pubkey,
    pub edition: u64,
}
