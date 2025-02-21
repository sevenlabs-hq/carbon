use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateMarginParametersArgs {
    pub future_margin_initial: u64,
    pub future_margin_maintenance: u64,
}
