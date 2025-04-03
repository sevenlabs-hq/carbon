use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
pub struct MetadataDelegateRecord {
    pub key: Key,
    pub bump: u8,
    pub mint: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
}
