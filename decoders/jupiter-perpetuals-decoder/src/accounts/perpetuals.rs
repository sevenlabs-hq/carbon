use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x1ca762bf68526cc4")]
pub struct Perpetuals {
    pub permissions: Permissions,
    pub pools: Vec<solana_sdk::pubkey::Pubkey>,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub transfer_authority_bump: u8,
    pub perpetuals_bump: u8,
    pub inception_time: i64,
}
