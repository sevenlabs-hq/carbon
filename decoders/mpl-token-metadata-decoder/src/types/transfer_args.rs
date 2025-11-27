use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum TransferArgs {
    V1 {
        amount: u64,
        authorization_data: Option<AuthorizationData>,
    },
}
