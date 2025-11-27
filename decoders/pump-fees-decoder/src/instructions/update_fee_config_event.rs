use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5a1741233ef4bcd0")]
pub struct UpdateFeeConfigEvent {
    pub timestamp: i64,
    pub admin: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
    pub fee_tiers: Vec<FeeTier>,
    pub flat_fees: Fees,
}
