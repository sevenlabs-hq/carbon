use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenSupplyParams {
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
}
