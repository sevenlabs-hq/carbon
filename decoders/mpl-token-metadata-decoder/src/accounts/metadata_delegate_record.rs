use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize)]
#[carbon(discriminator = "0xb994256b776af3ec")]
pub struct MetadataDelegateRecord {
    pub key: Key,
    pub bump: u8,
    pub mint: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
}
