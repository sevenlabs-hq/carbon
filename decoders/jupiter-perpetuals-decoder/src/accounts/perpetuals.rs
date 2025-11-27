use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1ca762bf68526cc4")]
pub struct Perpetuals {
    pub permissions: Permissions,
    pub pools: Vec<solana_pubkey::Pubkey>,
    pub admin: solana_pubkey::Pubkey,
    pub transfer_authority_bump: u8,
    pub perpetuals_bump: u8,
    pub inception_time: i64,
}
