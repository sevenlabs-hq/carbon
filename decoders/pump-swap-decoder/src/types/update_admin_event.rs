use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateAdminEvent {
    pub timestamp: i64,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub new_admin: solana_sdk::pubkey::Pubkey,
}
