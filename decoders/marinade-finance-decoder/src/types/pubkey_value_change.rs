use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PubkeyValueChange {
    pub old: solana_pubkey::Pubkey,
    pub new: solana_pubkey::Pubkey,
}
