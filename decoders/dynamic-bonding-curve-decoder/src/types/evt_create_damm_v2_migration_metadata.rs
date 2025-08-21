use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct EvtCreateDammV2MigrationMetadata {
    pub virtual_pool: solana_pubkey::Pubkey,
}
