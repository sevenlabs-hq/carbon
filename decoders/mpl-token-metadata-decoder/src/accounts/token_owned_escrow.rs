use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
pub struct TokenOwnedEscrow {
    pub key: Key,
    pub base_token: solana_pubkey::Pubkey,
    pub authority: EscrowAuthority,
    pub bump: u8,
}
