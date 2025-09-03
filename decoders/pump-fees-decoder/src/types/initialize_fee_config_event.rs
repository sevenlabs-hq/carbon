use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeFeeConfigEvent {
    pub timestamp: i64,
    pub admin: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
}
