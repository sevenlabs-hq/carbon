use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SlotFeeBracket {
    pub buy_fee_bps: u16,
    pub sell_fee_bps: u16,
    pub slot_offset_upperbound: u16,
}
