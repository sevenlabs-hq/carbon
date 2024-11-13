use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MasterEdition {
    pub max_supply: Option<u32>,
    pub name: Option<String>,
    pub uri: Option<String>,
}
