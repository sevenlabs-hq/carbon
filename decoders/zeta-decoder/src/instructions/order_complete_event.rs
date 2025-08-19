use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1a64c4ea5d799fdf")]
pub struct OrderCompleteEvent {
    pub margin_account: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub asset: Asset,
    pub market_index: u8,
    pub side: Side,
    pub unfilled_size: u64,
    pub order_id: u128,
    pub client_order_id: u64,
    pub order_complete_type: OrderCompleteType,
}
