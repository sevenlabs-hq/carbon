use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MetaplexMetadata {
    pub discriminator: u8,
    pub update_authority: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub data: MetaplexData,
    pub primary_sale_happened: bool,
    pub mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<MetaplexTokenStandard>,
    pub collection: Option<MetaplexCollection>,
    pub uses: Option<MetaplexUses>,
    pub collection_details: Option<MetaplexCollectionDetails>,
    pub programmable_config: Option<MetaplexProgrammableConfig>,
}
