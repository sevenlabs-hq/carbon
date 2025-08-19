use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeZetaGroupArgs {
    pub perps_only: bool,
    pub flex_underlying: bool,
    pub asset_override: Option<Asset>,
    pub zeta_group_nonce: u8,
    pub underlying_nonce: u8,
    pub greeks_nonce: u8,
    pub vault_nonce: u8,
    pub insurance_vault_nonce: u8,
    pub socialized_loss_account_nonce: u8,
    pub perp_sync_queue_nonce: u8,
    pub interest_rate: i64,
    pub volatility: [u64; 5],
    pub option_trade_normalizer: u64,
    pub future_trade_normalizer: u64,
    pub max_volatility_retreat: u64,
    pub max_interest_retreat: u64,
    pub max_delta: u64,
    pub min_delta: u64,
    pub min_interest_rate: i64,
    pub max_interest_rate: i64,
    pub min_volatility: u64,
    pub max_volatility: u64,
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
    pub expiry_interval_seconds: u32,
    pub new_expiry_threshold_seconds: u32,
    pub min_funding_rate_percent: i64,
    pub max_funding_rate_percent: i64,
    pub perp_impact_cash_delta: u64,
}
