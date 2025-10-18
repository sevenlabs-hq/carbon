use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ProtocolConfig {
    pub create_pool_fee: u64,
    pub initial_token_b_amount: f64,
    pub initial_token_a_amount: u64,
    pub unstaked_wsol_reserve: u64,
    pub total_sol_spent: u64,
    pub total_msol_received: u64,
    pub total_realized_profit: u64,
    pub pool_count: u64,
    pub max_supply_per_wallet: u64,
    pub creator_trading_fee_trading_volume_threshold: f64,
    pub market_cap_based_fees: LiquidityPoolMarketCapBasedFees,
    pub buffer_bps: u16,
    pub auto_staking_threshold_bps: u16,
    pub version: u16,
    pub protocol_config_state_bump: u8,
    pub allow_create_pool: u8,
    pub supported_pool_type: u8,
    pub default_leader_slot_window: u8,
    pub auto_staking_enabled: u8,
    pub leader_slot_window: u8,
    pub sandwich_resistence_enabled: u8,
    pub token_a_decimals: u8,
    pub migration_market_cap_threshold: u16,
    pub pad: [u8; 8],
    pub max_creator_trading_fee: u32,
    pub slot_offset_based_fees: LiquidityPoolSlotOffsetBasedFees,
}
