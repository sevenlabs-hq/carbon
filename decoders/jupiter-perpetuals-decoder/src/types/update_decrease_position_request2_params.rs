use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateDecreasePositionRequest2Params {
    pub size_usd_delta: u64,
    pub trigger_price: u64,
}
