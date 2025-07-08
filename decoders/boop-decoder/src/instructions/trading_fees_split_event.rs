use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d713c9f11fdae877a")]
pub struct TradingFeesSplitEvent {
    pub amount: u64,
    pub creator: solana_pubkey::Pubkey,
}
