use {
    alloc::string::String,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dffca4c935be74916")]
pub struct MigrationEvent {
    pub tokens_migrated: u64,
    pub tokens_burned: u64,
    pub collateral_migrated: u64,
    pub fee: u64,
    pub label: String,
}
