use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d38ded7eb4ec56392")]
pub struct InsuranceFundRecordEvent {
    pub ts: i64,
    pub spot_market_index: u16,
    pub perp_market_index: u16,
    pub user_if_factor: u32,
    pub total_if_factor: u32,
    pub vault_amount_before: u64,
    pub insurance_vault_amount_before: u64,
    pub total_if_shares_before: u128,
    pub total_if_shares_after: u128,
    pub amount: i64,
}
