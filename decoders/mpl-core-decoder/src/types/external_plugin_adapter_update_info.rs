
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum ExternalPluginAdapterUpdateInfo {
    LifecycleHook
                (
                    LifecycleHookUpdateInfo,
                )
    ,
    Oracle
                (
                    OracleUpdateInfo,
                )
    ,
    AppData
                (
                    AppDataUpdateInfo,
                )
    ,
    LinkedLifecycleHook
                (
                    LinkedLifecycleHookUpdateInfo,
                )
    ,
    LinkedAppData
                (
                    LinkedAppDataUpdateInfo,
                )
    ,
}


