use super::*;

use alloc::string::String;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlatformConfigInfo {
    pub fee_wallet: solana_pubkey::Pubkey,
    pub nft_wallet: solana_pubkey::Pubkey,
    pub migrate_nft_info: MigrateNftInfo,
    pub fee_rate: u64,
    pub name: String,
    pub web: String,
    pub img: String,
    pub transfer_fee_extension_auth: solana_pubkey::Pubkey,
    pub creator_fee_rate: u64,
}
