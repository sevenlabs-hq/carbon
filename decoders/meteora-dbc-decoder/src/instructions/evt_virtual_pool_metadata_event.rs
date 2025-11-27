use carbon_core::{borsh, CarbonDeserialize};
use solana_pubkey::Pubkey;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbc12484cc35b264a")]
pub struct EvtVirtualPoolMetadataEvent {
    pub virtual_pool_metadata: Pubkey,
    pub virtual_pool: Pubkey,
}
