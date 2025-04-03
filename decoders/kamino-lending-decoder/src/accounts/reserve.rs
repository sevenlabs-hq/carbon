use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x2bf2ccca1af73b7f")]
pub struct Reserve {
    pub version: u64,
    pub last_update: LastUpdate,
    pub lending_market: solana_pubkey::Pubkey,
    pub farm_collateral: solana_pubkey::Pubkey,
    pub farm_debt: solana_pubkey::Pubkey,
    pub liquidity: ReserveLiquidity,
    pub reserve_liquidity_padding: [u64; 150],
    pub collateral: ReserveCollateral,
    pub reserve_collateral_padding: [u64; 150],
    pub config: ReserveConfig,
    pub config_padding: [u64; 117],
    pub borrowed_amount_outside_elevation_group: u64,
    pub borrowed_amounts_against_this_reserve_in_elevation_groups: [u64; 32],
    pub padding: [u64; 207],
}
