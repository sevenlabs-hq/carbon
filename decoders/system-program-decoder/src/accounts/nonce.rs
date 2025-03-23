use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x216d14bf0b25e522")]
pub struct Nonce {
    pub version: NonceVersion,
    pub state: NonceState,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub blockhash: solana_sdk::pubkey::Pubkey,
    pub lamports_per_signature: u64,
}
