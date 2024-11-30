

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum Key {
    Uninitialized,
    AssetV1,
    HashedAssetV1,
    PluginHeaderV1,
    PluginRegistryV1,
    CollectionV1,
}


