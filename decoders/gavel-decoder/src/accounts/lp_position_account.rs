use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x65b11a2ca1f25788")]
pub struct LpPositionAccount {
    pub authority: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub status: u64,
    pub lp_position: LpPosition,
}
