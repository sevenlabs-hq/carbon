use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddPoolParams {
    pub name: String,
    pub limit: Limit,
    pub fees: Fees,
    pub max_request_execution_sec: i64,
}
