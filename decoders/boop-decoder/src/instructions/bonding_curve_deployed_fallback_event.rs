use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6afcf373c79ff71f")]
pub struct BondingCurveDeployedFallbackEvent {
    pub mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}
