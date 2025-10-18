use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AdminSetCreatorEvent {
    pub timestamp: i64,
    pub admin_set_creator_authority: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub old_creator: solana_pubkey::Pubkey,
    pub new_creator: solana_pubkey::Pubkey,
}
