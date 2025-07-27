use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd5e005d16245775c")]
pub struct VirtualPool {
    pub volatility_tracker: VolatilityTracker,
    pub config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub protocol_base_fee: u64,
    pub protocol_quote_fee: u64,
    pub trading_base_fee: u64,
    pub trading_quote_fee: u64,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub pool_type: u8,
    pub is_migrated: u8,
    pub is_partner_withdraw_surplus: u8,
    pub is_procotol_withdraw_surplus: u8,
    pub migration_progress: u8,
    pub padding_0: [u8; 3],
    pub metrics: PoolMetrics,
    pub finish_curve_timestamp: u64,
    pub padding_1: [u64; 9],
}
