use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MarginParameters {
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
}
