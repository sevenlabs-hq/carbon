

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TradingFeesSplitEvent {
    pub amount: u64,
    pub creator: solana_pubkey::Pubkey,
}
