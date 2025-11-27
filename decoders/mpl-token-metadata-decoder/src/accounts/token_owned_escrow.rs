use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenOwnedEscrow {
    pub key: Key,
    pub base_token: solana_pubkey::Pubkey,
    pub authority: EscrowAuthority,
    pub bump: u8,
}
