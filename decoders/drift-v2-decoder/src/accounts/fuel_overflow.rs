use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb640e7b1e28e453a")]
pub struct FuelOverflow {
    pub authority: solana_pubkey::Pubkey,
    pub fuel_insurance: u128,
    pub fuel_deposits: u128,
    pub fuel_borrows: u128,
    pub fuel_positions: u128,
    pub fuel_taker: u128,
    pub fuel_maker: u128,
    pub last_fuel_sweep_ts: u32,
    pub last_reset_ts: u32,
    pub padding: [u128; 6],
}
