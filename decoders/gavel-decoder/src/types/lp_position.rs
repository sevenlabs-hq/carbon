use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LpPosition {
    pub reward_factor_snapshot: u128,
    pub lp_shares: u64,
    pub withdrawable_lp_shares: u64,
    pub uncollected_fees: u64,
    pub collected_fees: u64,
    pub pending_shares_to_vest: PendingSharesToVest,
}
