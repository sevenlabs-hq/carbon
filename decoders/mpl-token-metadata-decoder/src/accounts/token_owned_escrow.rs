use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1589745b7b627ee4")]
pub struct TokenOwnedEscrow {
    pub key: Key,
    pub base_token: solana_pubkey::Pubkey,
    pub authority: EscrowAuthority,
    pub bump: u8,
}
