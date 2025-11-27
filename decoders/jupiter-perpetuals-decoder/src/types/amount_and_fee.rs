use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AmountAndFee {
    pub amount: u64,
    pub fee: u64,
    pub fee_bps: u64,
}
