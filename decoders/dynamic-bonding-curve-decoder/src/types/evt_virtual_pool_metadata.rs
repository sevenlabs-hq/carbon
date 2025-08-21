use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtVirtualPoolMetadata {
    pub virtual_pool_metadata: solana_pubkey::Pubkey,
    pub virtual_pool: solana_pubkey::Pubkey,
}
