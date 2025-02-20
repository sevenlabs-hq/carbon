use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct HistoricalOracleData {
    pub last_oracle_price: i64,
    pub last_oracle_conf: u64,
    pub last_oracle_delay: i64,
    pub last_oracle_price_twap: i64,
    pub last_oracle_price_twap5min: i64,
    pub last_oracle_price_twap_ts: i64,
}
