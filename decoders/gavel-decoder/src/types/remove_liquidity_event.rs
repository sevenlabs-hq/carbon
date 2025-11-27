use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RemoveLiquidityEvent {
    pub pool_total_lp_shares: u64,
    pub pool_total_base_liquidity: u64,
    pub pool_total_quote_liquitidy: u64,
    pub snapshot_base_liquidity: u64,
    pub snapshot_quote_liquidity: u64,
    pub user_lp_shares_burned: u64,
    pub user_lp_shares_available: u64,
    pub user_lp_shares_locked: u64,
    pub user_lp_shares_unlocked_for_withdrawal: u64,
    pub user_base_withdrawn: u64,
    pub user_quote_withdrawn: u64,
    pub user_total_withdrawable_base: u64,
    pub user_total_withdrawable_quote: u64,
}
