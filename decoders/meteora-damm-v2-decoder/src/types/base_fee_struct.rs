use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BaseFeeStruct {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding_0: [u8; 5],
    pub first_factor: u16,
    pub second_factor: [u8; 8],
    pub third_factor: u64,
    pub padding_1: u64,
}
