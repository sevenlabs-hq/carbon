use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x477605cb05628774")]
pub struct VirtualsPool {
    pub creator: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub virtual_y: u64,
    pub graduation_x: u64,
    pub state: PoolState,
    pub bump: u8,
}
