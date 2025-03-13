use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PubkeyValueChange {
    pub old: solana_sdk::pubkey::Pubkey,
    pub new: solana_sdk::pubkey::Pubkey,
}
