use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenRecord {
    pub key: Key,
    pub bump: u8,
    pub state: TokenState,
    pub rule_set_revision: Option<u64>,
    pub delegate: Option<solana_pubkey::Pubkey>,
    pub delegate_role: Option<TokenDelegateRole>,
    pub locked_transfer: Option<solana_pubkey::Pubkey>,
}
