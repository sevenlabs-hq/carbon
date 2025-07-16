use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5ca21a433156fc05")]
pub struct CrossMarginAccountManager {
    pub nonce: u8,
    pub authority: solana_pubkey::Pubkey,
    pub accounts: [CrossMarginAccountInfo; 20],
    pub referrer: solana_pubkey::Pubkey,
    pub airdrop_community: u8,
    pub referred_timestamp: u64,
    pub padding: [u8; 14],
}
