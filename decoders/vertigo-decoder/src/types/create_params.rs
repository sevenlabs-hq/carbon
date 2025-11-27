use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct CreateParams {
    pub shift: u128,
    pub initial_token_b_reserves: u64,
    pub fee_params: FeeParams,
}
