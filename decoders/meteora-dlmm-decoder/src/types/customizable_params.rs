use super::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CustomizableParams {
    pub active_id: i32,
    pub bin_step: u16,
    pub base_factor: u16,
    pub activation_type: u8,
    pub has_alpha_vault: bool,
    pub activation_point: Option<u64>,
    #[serde(with = "BigArray")]
    pub padding: [u8; 64],
}
