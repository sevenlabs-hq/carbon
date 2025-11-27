use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SpotFulfillmentMethod {
    ExternalMarket,
    Match(solana_pubkey::Pubkey, u16),
}
