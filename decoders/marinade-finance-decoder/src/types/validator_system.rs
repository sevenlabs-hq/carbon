use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ValidatorSystem {
    pub validator_list: List,
    pub manager_authority: solana_pubkey::Pubkey,
    pub total_validator_score: u32,
    pub total_active_balance: u64,
    pub auto_add_validator_enabled: u8,
}
