use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MetaplexCollection {
    pub verified: bool,
    pub key: solana_pubkey::Pubkey,
}
