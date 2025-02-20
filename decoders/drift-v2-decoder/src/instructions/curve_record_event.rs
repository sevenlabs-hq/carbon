use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d65ee28e4462e3d75")]
pub struct CurveRecordEvent {
    pub ts: i64,
    pub record_id: u64,
    pub peg_multiplier_before: u128,
    pub base_asset_reserve_before: u128,
    pub quote_asset_reserve_before: u128,
    pub sqrt_k_before: u128,
    pub peg_multiplier_after: u128,
    pub base_asset_reserve_after: u128,
    pub quote_asset_reserve_after: u128,
    pub sqrt_k_after: u128,
    pub base_asset_amount_long: u128,
    pub base_asset_amount_short: u128,
    pub base_asset_amount_with_amm: i128,
    pub total_fee: i128,
    pub total_fee_minus_distributions: i128,
    pub adjustment_cost: i128,
    pub oracle_price: i64,
    pub fill_record: u128,
    pub number_of_users: u32,
    pub market_index: u16,
}
