use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1db7bacbbae1bb5f82")]
pub struct SpotInterestRecordEvent {
    pub ts: i64,
    pub market_index: u16,
    pub deposit_balance: u128,
    pub cumulative_deposit_interest: u128,
    pub borrow_balance: u128,
    pub cumulative_borrow_interest: u128,
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
}
