use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MigrationFee {
    pub fee_percentage: u8,
    pub creator_fee_percentage: u8,
}
