use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ScopeConfiguration {
    pub price_feed: solana_pubkey::Pubkey,
    pub price_chain: [u16; 4],
    pub twap_chain: [u16; 4],
}
