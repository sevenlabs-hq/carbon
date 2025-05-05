use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TradingFeesSplitEvent {
    pub amount: u64,
    pub creator: solana_pubkey::Pubkey,
}
