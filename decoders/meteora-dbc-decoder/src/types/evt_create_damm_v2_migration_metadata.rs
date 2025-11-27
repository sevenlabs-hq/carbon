use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtCreateDammV2MigrationMetadata {
    pub virtual_pool: solana_pubkey::Pubkey,
}
