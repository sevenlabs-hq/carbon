use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OperatorSetPoolConfigParams {
    pub fees: Fees,
    pub limit: Limit,
    pub max_request_execution_sec: i64,
}
