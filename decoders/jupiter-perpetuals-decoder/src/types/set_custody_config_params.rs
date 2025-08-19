use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetCustodyConfigParams {
    pub oracle: OracleParams,
    pub pricing: PricingParams,
    pub permissions: Permissions,
    pub hourly_funding_dbps: u64,
    pub target_ratio_bps: u64,
    pub increase_position_bps: u64,
    pub decrease_position_bps: u64,
    pub doves_oracle: solana_pubkey::Pubkey,
    pub max_position_size_usd: u64,
    pub jump_rate: JumpRateState,
}
