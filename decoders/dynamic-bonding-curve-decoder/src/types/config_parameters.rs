use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ConfigParameters {
    pub pool_fees: PoolFeeParameters,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_type: u8,
    pub token_decimal: u8,
    pub partner_lp_percentage: u8,
    pub partner_locked_lp_percentage: u8,
    pub creator_lp_percentage: u8,
    pub creator_locked_lp_percentage: u8,
    pub migration_quote_threshold: u64,
    pub sqrt_start_price: u128,
    pub locked_vesting: LockedVestingParams,
    pub migration_fee_option: u8,
    pub token_supply: Option<TokenSupplyParams>,
    pub creator_trading_fee_percentage: u8,
    pub token_update_authority: u8,
    pub migration_fee: MigrationFee,
    pub migrated_pool_fee: MigratedPoolFee,
    pub padding: [u64; 7],
    pub curve: Vec<LiquidityDistributionParameters>,
}
