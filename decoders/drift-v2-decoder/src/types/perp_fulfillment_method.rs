use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PerpFulfillmentMethod {
    AMM(Option<u64>),
    Match(solana_pubkey::Pubkey, u16, u64),
}
