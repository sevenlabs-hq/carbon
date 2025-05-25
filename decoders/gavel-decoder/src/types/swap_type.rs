use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SwapType {
    ExactIn { amount_in: u64, min_amount_out: u64 },
    ExactOut { amount_out: u64, max_amount_in: u64 },
}
