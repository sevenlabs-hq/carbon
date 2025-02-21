use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1d4ea505f6a75bf4")]
pub struct SettleCpFeeEvent {
    pub delta_amount: u64,
    pub unclaimed_amount: u64,
    pub locked_amount: u64,
    pub curr_pool_lp: u64,
    pub last_pool_lp: u64,
    pub curr_k: u128,
    pub last_k: u128,
}
