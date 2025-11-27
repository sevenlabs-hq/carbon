use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d02971253cc865cbf")]
pub struct PoolEnabledEvent {
    pub pool: solana_pubkey::Pubkey,
    pub enabled: bool,
}
