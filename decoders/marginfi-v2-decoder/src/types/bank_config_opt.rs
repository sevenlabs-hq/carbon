
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct BankConfigOpt {
    pub asset_weight_init: Option<WrappedI80F48>,
    pub asset_weight_maint: Option<WrappedI80F48>,
    pub liability_weight_init: Option<WrappedI80F48>,
    pub liability_weight_maint: Option<WrappedI80F48>,
    pub deposit_limit: Option<u64>,
    pub borrow_limit: Option<u64>,
    pub operational_state: Option<BankOperationalState>,
    pub oracle: Option<OracleConfig>,
    pub interest_rate_config: Option<InterestRateConfigOpt>,
    pub risk_tier: Option<RiskTier>,
    pub total_asset_value_init_limit: Option<u64>,
}
