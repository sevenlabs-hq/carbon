use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExternalPluginAdapterUpdateInfo {
    LifecycleHook(LifecycleHookUpdateInfo),
    Oracle(OracleUpdateInfo),
    AppData(AppDataUpdateInfo),
    LinkedLifecycleHook(LinkedLifecycleHookUpdateInfo),
    LinkedAppData(LinkedAppDataUpdateInfo),
}
