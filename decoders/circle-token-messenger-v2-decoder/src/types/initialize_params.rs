use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeParams {
    pub token_controller: solana_pubkey::Pubkey,
    pub denylister: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub min_fee_controller: solana_pubkey::Pubkey,
    pub min_fee: u32,
    pub message_body_version: u32,
}
