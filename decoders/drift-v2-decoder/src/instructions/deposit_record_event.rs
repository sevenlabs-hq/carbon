use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1db4f1dacf66872c86")]
pub struct DepositRecordEvent {
    pub ts: i64,
    pub user_authority: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub direction: DepositDirection,
    pub deposit_record_id: u64,
    pub amount: u64,
    pub market_index: u16,
    pub oracle_price: i64,
    pub market_deposit_balance: u128,
    pub market_withdraw_balance: u128,
    pub market_cumulative_deposit_interest: u128,
    pub market_cumulative_borrow_interest: u128,
    pub total_deposits_after: u64,
    pub total_withdraws_after: u64,
    pub explanation: DepositExplanation,
    pub transfer_user: Option<solana_pubkey::Pubkey>,
}
