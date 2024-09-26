
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use crate::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d7ef0afce9e58996b")]
pub struct LiquidityChangeEvent{
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub tick: i32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity_before: u128,
    pub liquidity_after: u128,
}
