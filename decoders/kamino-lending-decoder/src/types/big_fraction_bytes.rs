use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BigFractionBytes {
    pub value: [u64; 4],
    pub padding: [u64; 2],
}
