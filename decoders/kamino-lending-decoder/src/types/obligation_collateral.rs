use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ObligationCollateral {
    pub deposit_reserve: solana_pubkey::Pubkey,
    pub deposited_amount: u64,
    pub market_value_sf: u128,
    pub borrowed_amount_against_this_collateral_in_elevation_group: u64,
    pub padding: [u64; 9],
}
