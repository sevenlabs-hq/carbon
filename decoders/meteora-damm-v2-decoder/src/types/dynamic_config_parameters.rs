use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DynamicConfigParameters {
    pub pool_creator_authority: solana_pubkey::Pubkey,
}
