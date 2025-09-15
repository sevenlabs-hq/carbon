use super::*;

use carbon_core::{
    borsh::{self, maybestd::vec::Vec},
    CarbonDeserialize,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct FeeConfig {
    pub bump: u8,
    pub admin: solana_pubkey::Pubkey,
    pub flat_fees: Fees,
    pub fee_tiers: Vec<FeeTier>,
}
