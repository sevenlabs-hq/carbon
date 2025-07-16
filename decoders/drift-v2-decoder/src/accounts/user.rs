use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9f755fe3ef973aec")]
pub struct User {
    pub authority: solana_pubkey::Pubkey,
    pub delegate: solana_pubkey::Pubkey,
    pub name: [u8; 32],
    pub spot_positions: [SpotPosition; 8],
    pub perp_positions: [PerpPosition; 8],
    pub orders: [Order; 32],
    pub last_add_perp_lp_shares_ts: i64,
    pub total_deposits: u64,
    pub total_withdraws: u64,
    pub total_social_loss: u64,
    pub settled_perp_pnl: i64,
    pub cumulative_spot_fees: i64,
    pub cumulative_perp_funding: i64,
    pub liquidation_margin_freed: u64,
    pub last_active_slot: u64,
    pub next_order_id: u32,
    pub max_margin_ratio: u32,
    pub next_liquidation_id: u16,
    pub sub_account_id: u16,
    pub status: u8,
    pub is_margin_trading_enabled: bool,
    pub idle: bool,
    pub open_orders: u8,
    pub has_open_order: bool,
    pub open_auctions: u8,
    pub has_open_auction: bool,
    pub margin_mode: MarginMode,
    pub pool_id: u8,
    pub padding1: [u8; 3],
    pub last_fuel_bonus_update_ts: u32,
    pub padding: [u8; 12],
}
