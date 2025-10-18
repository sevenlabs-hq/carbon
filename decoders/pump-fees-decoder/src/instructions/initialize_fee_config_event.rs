use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d598af4e60a38e27e")]
pub struct InitializeFeeConfigEvent {
    pub timestamp: i64,
    pub admin: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
}
