use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8f3492bbdb7b4c9b")]
pub struct FeeConfig {
    pub bump: u8,
    pub admin: solana_pubkey::Pubkey,
    pub flat_fees: Fees,
    pub fee_tiers: Vec<FeeTier>,
}
