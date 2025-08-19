use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d72a23b2154866c3e")]
pub struct TradeEventV3Event {
    pub margin_account: solana_pubkey::Pubkey,
    pub index: u8,
    pub size: u64,
    pub cost_of_trades: u64,
    pub is_bid: bool,
    pub client_order_id: u64,
    pub order_id: u128,
    pub asset: Asset,
    pub user: solana_pubkey::Pubkey,
    pub is_taker: bool,
    pub sequence_number: u64,
    pub fee: u64,
    pub price: u64,
    pub pnl: i64,
    pub rebate: u64,
}
