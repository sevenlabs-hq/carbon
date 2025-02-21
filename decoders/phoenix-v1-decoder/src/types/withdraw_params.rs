use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WithdrawParams {
    pub quote_lots_to_withdraw: Option<u64>,
    pub base_lots_to_withdraw: Option<u64>,
}
