use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CollectionAuthorityRecord {
    pub key: Key,
    pub bump: u8,
    pub update_authority: Option<solana_pubkey::Pubkey>,
}
