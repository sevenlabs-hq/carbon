use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeOrderParams {
    pub unique_id: u64,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expired_at: Option<i64>,
    pub fee_bps: Option<u16>,
}
