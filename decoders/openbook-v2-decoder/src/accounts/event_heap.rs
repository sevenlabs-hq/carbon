use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x773b3d13a55439af")]
pub struct EventHeap {
    pub header: EventHeapHeader,
    pub nodes: [EventNode; 600],
    pub reserved: [u8; 64],
}
