use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d7ef0afce9e58996b")]
pub struct LiquidityChangeEvent {
    pub pool_state: solana_pubkey::Pubkey,
    pub tick: i32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity_before: u128,
    pub liquidity_after: u128,
}
