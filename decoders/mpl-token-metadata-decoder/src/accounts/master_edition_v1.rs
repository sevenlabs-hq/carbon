use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
pub struct MasterEditionV1 {
    pub key: Key,
    pub supply: u64,
    pub max_supply: Option<u64>,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
    pub one_time_printing_authorization_mint: solana_sdk::pubkey::Pubkey,
}
