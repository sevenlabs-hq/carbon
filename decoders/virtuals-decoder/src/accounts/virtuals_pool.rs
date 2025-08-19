use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x477605cb05628774")]
pub struct VirtualsPool {
    pub creator: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub virtual_y: u64,
    pub graduation_x: u64,
    pub state: PoolState,
    pub bump: u8,
}
