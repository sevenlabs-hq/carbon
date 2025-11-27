use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BankConfigCompact {
    pub asset_weight_init: WrappedI80F48,
    pub asset_weight_maint: WrappedI80F48,
    pub liability_weight_init: WrappedI80F48,
    pub liability_weight_maint: WrappedI80F48,
    pub deposit_limit: u64,
    pub interest_rate_config: InterestRateConfigCompact,
    pub operational_state: BankOperationalState,
    pub oracle_setup: OracleSetup,
    pub oracle_key: solana_pubkey::Pubkey,
    pub auto_padding_0: [u8; 6],
    pub borrow_limit: u64,
    pub risk_tier: RiskTier,
    pub auto_padding_1: [u8; 7],
    pub total_asset_value_init_limit: u64,
}
