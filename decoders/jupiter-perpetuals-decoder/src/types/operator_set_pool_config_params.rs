use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OperatorSetPoolConfigParams {
    pub fees: Fees,
    pub limit: Limit,
    pub max_request_execution_sec: i64,
}
