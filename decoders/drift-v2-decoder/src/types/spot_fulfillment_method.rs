use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SpotFulfillmentMethod {
    ExternalMarket,
    Match(solana_sdk::pubkey::Pubkey, u16),
}
