use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d15225c9ee01db4f3")]
pub struct ClosePositionRequestEvent {
    pub entire_position: Option<bool>,
    pub executed: bool,
    pub request_change: u8,
    pub request_type: u8,
    pub side: u8,
    pub position_request_key: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub amount: u64,
}
