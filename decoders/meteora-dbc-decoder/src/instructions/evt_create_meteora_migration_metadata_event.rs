use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d63a7853fd68faf8b")]
pub struct EvtCreateMeteoraMigrationMetadataEvent {
    pub virtual_pool: Pubkey,
}
