use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0adf0c2c6bf537f7")]
pub struct PerpMarket {
    pub pubkey: solana_pubkey::Pubkey,
    pub amm: AMM,
    pub pnl_pool: PoolBalance,
    pub name: [u8; 32],
    pub insurance_claim: InsuranceClaim,
    pub unrealized_pnl_max_imbalance: u64,
    pub expiry_ts: i64,
    pub expiry_price: i64,
    pub next_fill_record_id: u64,
    pub next_funding_rate_record_id: u64,
    pub next_curve_record_id: u64,
    pub imf_factor: u32,
    pub unrealized_pnl_imf_factor: u32,
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
    pub margin_ratio_initial: u32,
    pub margin_ratio_maintenance: u32,
    pub unrealized_pnl_initial_asset_weight: u32,
    pub unrealized_pnl_maintenance_asset_weight: u32,
    pub number_of_users_with_base: u32,
    pub number_of_users: u32,
    pub market_index: u16,
    pub status: MarketStatus,
    pub contract_type: ContractType,
    pub contract_tier: ContractTier,
    pub paused_operations: u8,
    pub quote_spot_market_index: u16,
    pub fee_adjustment: i16,
    pub fuel_boost_position: u8,
    pub fuel_boost_taker: u8,
    pub fuel_boost_maker: u8,
    pub pool_id: u8,
    pub high_leverage_margin_ratio_initial: u16,
    pub high_leverage_margin_ratio_maintenance: u16,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 38],
}
