use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenMessenger {
    pub denylister: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub pending_owner: solana_pubkey::Pubkey,
    pub message_body_version: u32,
    pub authority_bump: u8,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub min_fee_controller: solana_pubkey::Pubkey,
    pub min_fee: u32,
}
