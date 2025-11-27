use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Edition {
    pub key: Key,
    pub parent: solana_pubkey::Pubkey,
    pub edition: u64,
}
