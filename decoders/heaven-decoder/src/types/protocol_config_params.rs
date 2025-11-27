use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct ProtocolConfigParams {
    pub create_pool_fee: u64,
    pub allow_create_pool: bool,
    pub supported_pool_type: LiquidityPoolType,
    pub market_cap_based_fees: LiquidityPoolMarketCapBasedFees,
    pub initial_token_b_amount: f64,
    pub initial_token_a_amount: u64,
    pub default_leader_slot_window: u8,
    pub auto_staking_enabled: bool,
    pub sandwich_resistence_enabled: bool,
    pub buffer_bps: u16,
    pub auto_staking_threshold_bps: u16,
    pub token_a_decimals: u8,
    pub max_creator_trading_fee: u32,
    pub max_supply_per_wallet: u64,
    pub creator_trading_fee_trading_volume_threshold: f64,
    pub migration_market_cap_threshold: u16,
}
