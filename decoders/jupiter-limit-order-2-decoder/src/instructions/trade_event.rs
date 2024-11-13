use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbddb7fd34ee661ee")]
pub struct TradeEvent {
    pub order_key: solana_sdk::pubkey::Pubkey,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub remaining_making_amount: u64,
    pub remaining_taking_amount: u64,
    pub making_amount: u64,
    pub taking_amount: u64,
}
