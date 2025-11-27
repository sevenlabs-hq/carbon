use {
    super::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EventNode {
    pub next: u16,
    pub prev: u16,
    pub pad: [u8; 4],
    pub event: AnyEvent,
}
