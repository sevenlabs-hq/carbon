use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbc12484cc35b264a")]
pub struct EvtVirtualPoolMetadataEvent {
    pub virtual_pool_metadata: solana_pubkey::Pubkey,
    pub virtual_pool: solana_pubkey::Pubkey,
}
