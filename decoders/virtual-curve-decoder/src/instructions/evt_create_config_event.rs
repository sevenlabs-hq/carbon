use alloc::vec::Vec;
use carbon_core::{borsh, CarbonDeserialize};

use crate::types::{LiquidityDistributionParameters, PoolFeeParameters};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d83cfb4aeb449a536")]
pub struct EvtCreateConfigEvent {
    pub config: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub fee_claimer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub pool_fees: PoolFeeParameters,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_decimal: u8,
    pub token_type: u8,
    pub partner_locked_lp_percentage: u8,
    pub partner_lp_percentage: u8,
    pub creator_locked_lp_percentage: u8,
    pub creator_lp_percentage: u8,
    pub swap_base_amount: u64,
    pub migration_quote_threshold: u64,
    pub migration_base_amount: u64,
    pub sqrt_start_price: u128,
    pub curve: Vec<LiquidityDistributionParameters>,
}
