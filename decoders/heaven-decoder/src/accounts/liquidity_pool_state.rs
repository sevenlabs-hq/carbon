use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[carbon(discriminator = "0xbe9edc820fa284fc")]
pub struct LiquidityPoolState {
    pub info: LiquidityPoolInfo,
    pub market_cap_based_fees: LiquidityPoolMarketCapBasedFees,
    pub reserve: LiquidityPoolReserve,
    pub lp_token: LiquidityPoolLpTokenInfo,
    pub protocol_trading_fees: u64,
    pub creator_trading_fees: u64,
    pub creator_trading_fees_claimed_by_creator: u64,
    pub creator_trading_fees_claimed_by_others: u64,
    pub liquidity_provider_trading_fees: u64,
    pub creator_trading_fee_protocol_fees: u64,
    pub reflection_trading_fees: u64,
    pub created_at_slot: u64,
    pub trading_volume_usd: f64,
    pub creator_trading_fee_trading_volume_threshold: f64,
    pub creator_trading_fee_trading_volume_threshold_reached_unix_timestamp: u64,
    pub token_a_vault: solana_pubkey::Pubkey,
    pub token_b_vault: solana_pubkey::Pubkey,
    pub protocol_config: solana_pubkey::Pubkey,
    pub key: solana_pubkey::Pubkey,
    pub token_a: LiquidityPoolTokenInfo,
    pub token_b: LiquidityPoolTokenInfo,
    pub allowlist: LiquidityPoolAllowlist,
    pub feature_flags: LiquidityPoolFeatureFlags,
    pub taxable_side: u8,
    pub taxable_side_type: u8,
    pub creator_trading_fee_distribution: u8,
    pub creator_trading_fee_claim_status: u8,
    pub fee_configuration_mode: u8,
    pub is_migrated: u8,
    pub pad: [u8; 13],
    pub slot_offset_based_fees: LiquidityPoolSlotOffsetBasedFees,
    pub creator_trading_fee_receiver: solana_pubkey::Pubkey,
}
