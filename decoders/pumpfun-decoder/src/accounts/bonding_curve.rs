use carbon_core::{borsh, CarbonDeserialize};

use crate::types::OptionBool;

pub const BONDING_CURVE_LEN_PRE_MAYHEM_MODE: usize = 8 + (8 * 5) + 1 + 32;

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17b7f83760d8ac60")]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: solana_pubkey::Pubkey,
    pub is_mayhem_mode: OptionBool,
}
