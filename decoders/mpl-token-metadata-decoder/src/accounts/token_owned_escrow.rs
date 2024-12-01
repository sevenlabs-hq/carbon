use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct TokenOwnedEscrow {
    pub key: Key,
    pub base_token: solana_sdk::pubkey::Pubkey,
    pub authority: EscrowAuthority,
    pub bump: u8,
}
