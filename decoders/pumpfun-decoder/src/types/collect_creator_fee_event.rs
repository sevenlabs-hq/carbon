use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CollectCreatorFeeEvent {
    pub timestamp: i64,
    pub creator: solana_pubkey::Pubkey,
    pub creator_fee: u64,
}
