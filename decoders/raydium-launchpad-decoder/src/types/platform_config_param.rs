use super::*;

use alloc::{format, string::String};
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PlatformConfigParam {
    FeeWallet(solana_pubkey::Pubkey),
    NFTWallet(solana_pubkey::Pubkey),
    MigrateNftInfo(MigrateNftInfo),
    FeeRate(u64),
    Name(String),
    Web(String),
    Img(String),
    CpSwapConfig,
    AllInfo(PlatformConfigInfo),
}
