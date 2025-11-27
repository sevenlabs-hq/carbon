use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dcd355bef2288492f")]
pub struct StrategyDepositEvent {
    pub strategy_type: StrategyType,
    pub token_amount: u64,
}
