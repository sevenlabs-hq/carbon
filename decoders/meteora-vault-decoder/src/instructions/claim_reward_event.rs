use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d947486cc16ab555f")]
pub struct ClaimRewardEvent {
    pub strategy_type: StrategyType,
    pub token_amount: u64,
    pub mint_account: solana_pubkey::Pubkey,
}
