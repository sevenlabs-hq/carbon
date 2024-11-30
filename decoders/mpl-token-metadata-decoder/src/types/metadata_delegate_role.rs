

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum MetadataDelegateRole {
    AuthorityItem,
    Collection,
    Use,
    Data,
    ProgrammableConfig,
    DataItem,
    CollectionItem,
    ProgrammableConfigItem,
}


