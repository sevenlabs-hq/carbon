use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Vault {
    pub admin: solana_pubkey::Pubkey,
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub withdraw_authority_bump: u8,
    pub authority_bump: u8,
    pub is_active: bool,
    pub beneficiary: solana_pubkey::Pubkey,
    pub beneficiary_fee: u64,
    pub pending_admin: Option<solana_pubkey::Pubkey>,
}
