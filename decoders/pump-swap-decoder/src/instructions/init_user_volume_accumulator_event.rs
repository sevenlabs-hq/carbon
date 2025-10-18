use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d86240d48e86582d8")]
pub struct InitUserVolumeAccumulatorEvent {
    pub payer: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub timestamp: i64,
}
