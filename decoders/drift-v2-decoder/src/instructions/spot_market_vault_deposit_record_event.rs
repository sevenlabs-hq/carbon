use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1db2d917bc7fbe2049")]
pub struct SpotMarketVaultDepositRecordEvent {
    pub ts: i64,
    pub market_index: u16,
    pub deposit_balance: u128,
    pub cumulative_deposit_interest_before: u128,
    pub cumulative_deposit_interest_after: u128,
    pub deposit_token_amount_before: u64,
    pub amount: u64,
}
