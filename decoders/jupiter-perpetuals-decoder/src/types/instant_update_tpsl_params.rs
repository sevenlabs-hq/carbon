use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InstantUpdateTpslParams {
    pub size_usd_delta: u64,
    pub trigger_price: u64,
    pub request_time: i64,
}
