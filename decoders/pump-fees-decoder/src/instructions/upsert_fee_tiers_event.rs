use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dab59a9bb7aba21cc")]
pub struct UpsertFeeTiersEvent {
    pub timestamp: i64,
    pub admin: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
    pub fee_tiers: Vec<FeeTier>,
    pub offset: u8,
}
