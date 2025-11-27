use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbddb7fd34ee661ee")]
pub struct TradeEvent {
    pub margin_account: solana_pubkey::Pubkey,
    pub index: u8,
    pub size: u64,
    pub cost_of_trades: u64,
    pub is_bid: bool,
    pub client_order_id: u64,
    pub order_id: u128,
}
