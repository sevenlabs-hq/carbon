use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AmountAndFee {
    pub amount: u64,
    pub fee: u64,
    pub fee_bps: u64,
}
