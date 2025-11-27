use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeFeeConfigEvent {
    pub timestamp: i64,
    pub admin: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
}
