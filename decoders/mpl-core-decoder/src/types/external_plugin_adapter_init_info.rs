
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum ExternalPluginAdapterInitInfo {
    LifecycleHook
                (
                    LifecycleHookInitInfo,
                )
    ,
    Oracle
                (
                    OracleInitInfo,
                )
    ,
    AppData
                (
                    AppDataInitInfo,
                )
    ,
    LinkedLifecycleHook
                (
                    LinkedLifecycleHookInitInfo,
                )
    ,
    LinkedAppData
                (
                    LinkedAppDataInitInfo,
                )
    ,
    DataSection
                (
                    DataSectionInitInfo,
                )
    ,
}


