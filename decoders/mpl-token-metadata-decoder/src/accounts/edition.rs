use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
pub struct Edition {
    pub key: Key,
    pub parent: solana_sdk::pubkey::Pubkey,
    pub edition: u64,
}
