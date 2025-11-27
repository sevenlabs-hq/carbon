use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa204f23493f3dd60")]
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
