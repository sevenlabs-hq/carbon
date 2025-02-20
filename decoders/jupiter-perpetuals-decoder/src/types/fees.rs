use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Fees {
    pub swap_multiplier: u64,
    pub stable_swap_multiplier: u64,
    pub add_remove_liquidity_bps: u64,
    pub swap_bps: u64,
    pub tax_bps: u64,
    pub stable_swap_bps: u64,
    pub stable_swap_tax_bps: u64,
    pub liquidation_reward_bps: u64,
    pub protocol_share_bps: u64,
}
