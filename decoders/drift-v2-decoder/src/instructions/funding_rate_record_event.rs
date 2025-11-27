use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d4403ff1a855b93fe")]
pub struct FundingRateRecordEvent {
    pub ts: i64,
    pub record_id: u64,
    pub market_index: u16,
    pub funding_rate: i64,
    pub funding_rate_long: i128,
    pub funding_rate_short: i128,
    pub cumulative_funding_rate_long: i128,
    pub cumulative_funding_rate_short: i128,
    pub oracle_price_twap: i64,
    pub mark_price_twap: u64,
    pub period_revenue: i64,
    pub base_asset_amount_with_amm: i128,
    pub base_asset_amount_with_unsettled_lp: i128,
}
