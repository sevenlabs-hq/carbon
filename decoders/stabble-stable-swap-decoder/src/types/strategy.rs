use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Strategy {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub is_active: bool,
    pub amp_min_factor: u16,
    pub amp_max_factor: u16,
    pub ramp_min_step: u16,
    pub ramp_max_step: u16,
    pub ramp_min_duration: u32,
    pub ramp_max_duration: u32,
}
