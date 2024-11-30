
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct OracleInitInfo {
    pub base_address: solana_sdk::pubkey::Pubkey,
    pub init_plugin_authority: Option<Authority>,
    pub lifecycle_checks: Vec<(HookableLifecycleEvent, ExternalCheckResult)>,
    pub base_address_config: Option<ExtraAccount>,
    pub results_offset: Option<ValidationResultsOffset>,
}
