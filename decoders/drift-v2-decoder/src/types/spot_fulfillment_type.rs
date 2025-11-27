use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SpotFulfillmentType {
    SerumV3,
    Match,
    PhoenixV1,
    OpenbookV2,
}
