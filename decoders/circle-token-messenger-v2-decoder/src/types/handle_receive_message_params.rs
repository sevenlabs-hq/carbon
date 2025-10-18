use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct HandleReceiveMessageParams {
    pub remote_domain: u32,
    pub sender: solana_pubkey::Pubkey,
    pub finality_threshold_executed: u32,
    pub message_body: Vec<u8>,
    pub authority_bump: u8,
}
