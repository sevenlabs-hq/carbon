
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LifecycleHookInitInfo {
    pub hooked_program: solana_sdk::pubkey::Pubkey,
    pub init_plugin_authority: Option<Authority>,
    pub lifecycle_checks: Vec<(HookableLifecycleEvent, ExternalCheckResult)>,
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    pub data_authority: Option<Authority>,
    pub schema: Option<ExternalPluginAdapterSchema>,
}
