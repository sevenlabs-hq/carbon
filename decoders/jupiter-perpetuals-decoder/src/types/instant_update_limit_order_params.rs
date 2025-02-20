use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InstantUpdateLimitOrderParams {
    pub size_usd_delta: u64,
    pub trigger_price: u64,
    pub request_time: i64,
}
