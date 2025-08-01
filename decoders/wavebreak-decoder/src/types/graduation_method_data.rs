use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GraduationMethodData {
    pub label: GraduationMethodLabel,
    pub graduated: bool,
    pub fee_tier_index: u16,
    pub split_bps: u16,
    pub destination: solana_pubkey::Pubkey,
    pub unlocked: bool,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 89],
}
