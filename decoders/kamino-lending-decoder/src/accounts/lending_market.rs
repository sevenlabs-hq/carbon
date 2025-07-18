use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf6723262489d1c78")]
pub struct LendingMarket {
    pub version: u64,
    pub bump_seed: u64,
    pub lending_market_owner: solana_pubkey::Pubkey,
    pub lending_market_owner_cached: solana_pubkey::Pubkey,
    pub quote_currency: [u8; 32],
    pub referral_fee_bps: u16,
    pub emergency_mode: u8,
    pub autodeleverage_enabled: u8,
    pub borrow_disabled: u8,
    pub price_refresh_trigger_to_max_age_pct: u8,
    pub liquidation_max_debt_close_factor_pct: u8,
    pub insolvency_risk_unhealthy_ltv_pct: u8,
    pub min_full_liquidation_value_threshold: u64,
    pub max_liquidatable_debt_market_value_at_once: u64,
    pub global_unhealthy_borrow_value: u64,
    pub global_allowed_borrow_value: u64,
    pub risk_council: solana_pubkey::Pubkey,
    pub reserved1: [u8; 8],
    pub elevation_groups: [ElevationGroup; 32],
    #[serde(with = "serde_big_array::BigArray")]
    pub elevation_group_padding: [u64; 90],
    pub min_net_value_in_obligation_sf: u128,
    pub min_value_skip_liquidation_ltv_bf_checks: u64,
    pub name: [u8; 32],
    #[serde(with = "serde_big_array::BigArray")]
    pub padding1: [u64; 173],
}
