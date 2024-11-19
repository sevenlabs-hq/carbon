

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SwitchboardConfiguration {
    pub price_aggregator: solana_sdk::pubkey::Pubkey,
    pub twap_aggregator: solana_sdk::pubkey::Pubkey,
}
