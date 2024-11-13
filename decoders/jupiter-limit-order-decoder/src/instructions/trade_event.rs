use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbddb7fd34ee661ee")]
pub struct TradeEvent {
    pub order_key: solana_sdk::pubkey::Pubkey,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub remaining_in_amount: u64,
    pub remaining_out_amount: u64,
    pub in_amount: u64,
    pub out_amount: u64,
}
