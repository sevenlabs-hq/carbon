use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Seat {
    pub discriminant: u64,
    pub market: solana_sdk::pubkey::Pubkey,
    pub trader: solana_sdk::pubkey::Pubkey,
    pub approval_status: u64,
    pub padding: [u64; 6],
}
