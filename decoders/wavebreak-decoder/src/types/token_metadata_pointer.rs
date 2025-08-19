use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenMetadataPointer {
    pub update_authority: solana_pubkey::Pubkey,
    pub metadata_address: solana_pubkey::Pubkey,
}
