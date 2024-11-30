use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct Edition {
    pub key: Key,
    pub parent: solana_sdk::pubkey::Pubkey,
    pub edition: u64,
}
