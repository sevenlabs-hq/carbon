

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TokenSupplyParams {
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
}
