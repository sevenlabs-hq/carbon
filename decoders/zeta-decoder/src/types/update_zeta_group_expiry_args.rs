use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateZetaGroupExpiryArgs {
    pub expiry_interval_seconds: u32,
    pub new_expiry_threshold_seconds: u32,
}
