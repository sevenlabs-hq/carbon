use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LifecycleHookUpdateInfo {
    pub lifecycle_checks: Option<Vec<(HookableLifecycleEvent, ExternalCheckResult)>>,
    pub extra_accounts: Option<Vec<ExtraAccount>>,
    pub schema: Option<ExternalPluginAdapterSchema>,
}
