use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PoolApr {
    pub last_updated: i64,
    pub fee_apr_bps: u64,
    pub realized_fee_usd: u64,
}
