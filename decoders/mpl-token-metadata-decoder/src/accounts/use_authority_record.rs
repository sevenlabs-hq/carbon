use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UseAuthorityRecord {
    pub key: Key,
    pub allowed_uses: u64,
    pub bump: u8,
}
