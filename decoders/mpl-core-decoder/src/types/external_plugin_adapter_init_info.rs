use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExternalPluginAdapterInitInfo {
    LifecycleHook(LifecycleHookInitInfo),
    Oracle(OracleInitInfo),
    AppData(AppDataInitInfo),
    LinkedLifecycleHook(LinkedLifecycleHookInitInfo),
    LinkedAppData(LinkedAppDataInitInfo),
    DataSection(DataSectionInitInfo),
}
