use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisplayOptions {
    pub show_collection_metadata: bool,
    pub show_grand_total: bool,
    pub show_unverified_collections: bool,
    pub show_raw_data: bool,
    pub show_fungible: bool,
    pub require_full_index: bool,
    pub show_system_metadata: bool,
    pub show_zero_balance: bool,
    pub show_closed_accounts: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetOptions {
    pub show_collection_metadata: bool,
    pub show_unverified_collections: bool,
    pub show_raw_data: bool,
    pub show_fungible: bool,
    pub require_full_index: bool,
    pub show_system_metadata: bool,
    pub show_native_balance: bool,
    pub show_inscription: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SearchAssetsOptions {
    // Customer configured
    #[serde(default)]
    pub show_collection_metadata: bool,
    #[serde(default)]
    pub show_grand_total: bool,
    #[serde(default)]
    pub show_unverified_collections: bool,
    #[serde(default)]
    pub show_raw_data: bool,
    #[serde(default)]
    pub require_full_index: bool,
    #[serde(default)]
    pub show_system_metadata: bool,
    #[serde(default)]
    pub show_zero_balance: bool,
    #[serde(default)]
    pub show_closed_accounts: bool,
    #[serde(default)]
    pub show_native_balance: bool,
}
