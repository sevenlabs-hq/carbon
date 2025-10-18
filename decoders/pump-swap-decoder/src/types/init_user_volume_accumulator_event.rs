use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitUserVolumeAccumulatorEvent {
    pub payer: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub timestamp: i64,
}
