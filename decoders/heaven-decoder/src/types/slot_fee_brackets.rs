use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SlotFeeBrackets {
    #[serde(with = "serde_big_array::BigArray")]
    pub brackets: [SlotFeeBracket; 42],
    pub max_slot_offset: u16,
    pub max_fee_bps: u16,
    pub count: u8,
    pub enabled: u8,
    pub padding: [u8; 4],
}
