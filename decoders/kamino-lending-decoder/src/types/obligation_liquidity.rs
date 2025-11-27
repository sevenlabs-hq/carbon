use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ObligationLiquidity {
    pub borrow_reserve: solana_pubkey::Pubkey,
    pub cumulative_borrow_rate_bsf: BigFractionBytes,
    pub padding: u64,
    pub borrowed_amount_sf: u128,
    pub market_value_sf: u128,
    pub borrow_factor_adjusted_market_value_sf: u128,
    pub borrowed_amount_outside_elevation_groups: u64,
    pub padding2: [u64; 7],
}
