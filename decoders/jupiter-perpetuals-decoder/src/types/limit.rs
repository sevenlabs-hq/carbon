use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Limit {
    pub max_aum_usd: u128,
    pub token_weightage_buffer_bps: u128,
    pub buffer: u64,
}
