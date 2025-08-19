use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1a6c0e7b74e6812b")]
pub struct PoolConfig {
    pub quote_mint: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub pool_fees: PoolFeesConfig,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_decimal: u8,
    pub version: u8,
    pub token_type: u8,
    pub quote_token_flag: u8,
    pub partner_locked_lp_percentage: u8,
    pub partner_lp_percentage: u8,
    pub creator_locked_lp_percentage: u8,
    pub creator_lp_percentage: u8,
    pub migration_fee_option: u8,
    pub padding_0: [u8; 4],
    pub padding_1: [u8; 8],
    pub swap_base_amount: u64,
    pub migration_quote_threshold: u64,
    pub migration_base_threshold: u64,
    pub migration_sqrt_price: u128,
    pub locked_vesting_config: LockedVestingConfig,
    pub padding_2: [u128; 3],
    pub sqrt_start_price: u128,
    pub curve: [LiquidityDistributionConfig; 20],
}
