use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x773b3d13a55439af")]
pub struct EventHeap {
    pub header: EventHeapHeader,
    #[serde(with = "serde_big_array::BigArray")]
    pub nodes: [EventNode; 600],
    #[serde(with = "serde_big_array::BigArray")]
    pub reserved: [u8; 64],
}
