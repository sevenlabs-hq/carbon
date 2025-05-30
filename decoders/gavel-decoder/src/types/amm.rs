use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Amm {
    pub fee_in_bps: u32,
    pub protocol_allocation_in_pct: u32,
    pub lp_vesting_window: u64,
    pub reward_factor: u128,
    pub total_lp_shares: u64,
    pub slot_snapshot: u64,
    pub base_reserves_snapshot: u64,
    pub quote_reserves_snapshot: u64,
    pub base_reserves: u64,
    pub quote_reserves: u64,
    pub cumulative_quote_lp_fees: u64,
    pub cumulative_quote_protocol_fees: u64,
}
