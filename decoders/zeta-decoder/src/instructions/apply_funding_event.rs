use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d7fca0fb7c8c0040c")]
pub struct ApplyFundingEvent {
    pub margin_account: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub asset: Asset,
    pub balance_change: i64,
    pub remaining_balance: u64,
    pub funding_rate: i64,
    pub oracle_price: u64,
    pub position_size: i64,
}
