use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d676f84a88cfd9672")]
pub struct EvtCreateDammV2MigrationMetadataEvent {
    pub virtual_pool: Pubkey,
}
