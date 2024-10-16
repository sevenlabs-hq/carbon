use carbon_core::borsh;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3ecdf2aff4a98834")]
pub struct Deposit {
    pub log_type: u8,
    pub max_coin: u64,
    pub max_pc: u64,
    pub base: u64,
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub pool_lp: u64,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    pub deduct_coin: u64,
    pub deduct_pc: u64,
    pub mint_lp: u64,
}
