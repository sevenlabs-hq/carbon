use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GraduationEvent {
    pub vpool: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub balance: u64,
}
