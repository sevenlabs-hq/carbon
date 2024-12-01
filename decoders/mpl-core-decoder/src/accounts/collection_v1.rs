use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct CollectionV1 {
    pub key: Key,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub name: String,
    pub uri: String,
    pub num_minted: u32,
    pub current_size: u32,
}
