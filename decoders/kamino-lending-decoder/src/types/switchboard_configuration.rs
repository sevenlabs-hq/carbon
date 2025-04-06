use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwitchboardConfiguration {
    pub price_aggregator: solana_pubkey::Pubkey,
    pub twap_aggregator: solana_pubkey::Pubkey,
}
