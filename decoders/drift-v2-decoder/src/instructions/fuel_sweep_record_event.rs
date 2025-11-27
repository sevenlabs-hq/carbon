use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d295425f684f08308")]
pub struct FuelSweepRecordEvent {
    pub ts: i64,
    pub authority: solana_pubkey::Pubkey,
    pub user_stats_fuel_insurance: u32,
    pub user_stats_fuel_deposits: u32,
    pub user_stats_fuel_borrows: u32,
    pub user_stats_fuel_positions: u32,
    pub user_stats_fuel_taker: u32,
    pub user_stats_fuel_maker: u32,
    pub fuel_overflow_fuel_insurance: u128,
    pub fuel_overflow_fuel_deposits: u128,
    pub fuel_overflow_fuel_borrows: u128,
    pub fuel_overflow_fuel_positions: u128,
    pub fuel_overflow_fuel_taker: u128,
    pub fuel_overflow_fuel_maker: u128,
}
