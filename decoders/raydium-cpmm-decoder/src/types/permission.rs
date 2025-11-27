use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Permission {
    pub authority: solana_pubkey::Pubkey,
    pub padding: [u64; 30],
}
