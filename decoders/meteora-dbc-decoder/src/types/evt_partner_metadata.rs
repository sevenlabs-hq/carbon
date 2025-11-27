use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtPartnerMetadata {
    pub partner_metadata: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
}
