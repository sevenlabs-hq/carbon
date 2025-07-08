use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1db9249c52bda4cf4f")]
pub struct BondingCurveVaultClosedEvent {
    pub mint: solana_pubkey::Pubkey,
    pub recipient: solana_pubkey::Pubkey,
    pub amount: u64,
}
