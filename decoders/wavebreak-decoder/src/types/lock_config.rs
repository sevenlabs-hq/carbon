use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LockConfig {
    pub discriminator: [u8; 8],
    pub position: solana_pubkey::Pubkey,
    pub position_owner: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub locked_timestamp: u64,
    pub lock_type: LockTypeLabel,
}
