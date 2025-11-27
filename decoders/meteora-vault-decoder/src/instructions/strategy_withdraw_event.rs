use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d784cd05fddd2e5bd")]
pub struct StrategyWithdrawEvent {
    pub strategy_type: StrategyType,
    pub collateral_amount: u64,
    pub estimated_token_amount: u64,
}
