use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ExternalPluginAdapterType {
    LifecycleHook,
    Oracle,
    AppData,
    LinkedLifecycleHook,
    LinkedAppData,
    DataSection,
}
