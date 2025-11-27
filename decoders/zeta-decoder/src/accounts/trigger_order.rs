use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xec3d2abe980c6a74")]
pub struct TriggerOrder {
    pub owner: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub order_price: u64,
    pub trigger_price: Option<u64>,
    pub trigger_ts: Option<u64>,
    pub size: u64,
    pub creation_ts: u64,
    pub trigger_direction: Option<TriggerDirection>,
    pub side: Side,
    pub asset: Asset,
    pub order_type: OrderType,
    pub bit: u8,
    pub reduce_only: bool,
}
