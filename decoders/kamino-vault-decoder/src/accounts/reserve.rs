use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2bf2ccca1af73b7f")]
pub struct Reserve {
    pub version: u64,
    pub last_update: LastUpdate,
    pub lending_market: solana_pubkey::Pubkey,
    pub farm_collateral: solana_pubkey::Pubkey,
    pub farm_debt: solana_pubkey::Pubkey,
    pub liquidity: ReserveLiquidity,
    #[serde(with = "serde_big_array::BigArray")]
    pub reserve_liquidity_padding: [u64; 150],
    pub collateral: ReserveCollateral,
    #[serde(with = "serde_big_array::BigArray")]
    pub reserve_collateral_padding: [u64; 150],
    pub config: ReserveConfig,
    #[serde(with = "serde_big_array::BigArray")]
    pub config_padding: [u64; 116],
    pub borrowed_amount_outside_elevation_group: u64,
    pub borrowed_amounts_against_this_reserve_in_elevation_groups: [u64; 32],
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u64; 207],
}
