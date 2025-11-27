use {
    carbon_core::{CarbonDeserialize, borsh},
    serde_big_array::BigArray,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AnyEvent {
    pub event_type: u8,
    #[serde(with = "BigArray")]
    pub padding: [u8; 143],
}
