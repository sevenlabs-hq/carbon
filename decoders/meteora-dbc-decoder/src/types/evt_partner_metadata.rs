use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtPartnerMetadata {
    pub partner_metadata: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
}
