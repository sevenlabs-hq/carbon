use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AuditLogHeader {
    pub instruction: u8,
    pub sequence_number: u64,
    pub timestamp: i64,
    pub slot: u64,
    pub market: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub total_events: u16,
}
