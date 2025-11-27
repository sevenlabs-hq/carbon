use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExternalPluginAdapter {
    LifecycleHook(LifecycleHook),
    Oracle(Oracle),
    AppData(AppData),
    LinkedLifecycleHook(LinkedLifecycleHook),
    LinkedAppData(LinkedAppData),
    DataSection(DataSection),
}
