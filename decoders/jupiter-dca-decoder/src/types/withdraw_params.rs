use {
    super::*,
    carbon_core::{CarbonDeserialize, borsh},
};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WithdrawParams {
    pub withdraw_amount: u64,
    pub withdrawal: Withdrawal,
}
