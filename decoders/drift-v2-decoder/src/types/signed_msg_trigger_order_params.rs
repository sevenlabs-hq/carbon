use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SignedMsgTriggerOrderParams {
    pub trigger_price: u64,
    pub base_asset_amount: u64,
}
