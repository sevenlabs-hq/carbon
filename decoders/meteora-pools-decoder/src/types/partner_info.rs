use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PartnerInfo {
    pub fee_numerator: u64,
    pub partner_authority: solana_pubkey::Pubkey,
    pub pending_fee_a: u64,
    pub pending_fee_b: u64,
}
