use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d44429c07d894fa72")]
pub struct InsuranceFundStakeRecordEvent {
    pub ts: i64,
    pub user_authority: solana_pubkey::Pubkey,
    pub action: StakeAction,
    pub amount: u64,
    pub market_index: u16,
    pub insurance_vault_amount_before: u64,
    pub if_shares_before: u128,
    pub user_if_shares_before: u128,
    pub total_if_shares_before: u128,
    pub if_shares_after: u128,
    pub user_if_shares_after: u128,
    pub total_if_shares_after: u128,
}
