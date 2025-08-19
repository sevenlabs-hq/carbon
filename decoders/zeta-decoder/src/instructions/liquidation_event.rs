use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d030d155dad884890")]
pub struct LiquidationEvent {
    pub liquidator_reward: u64,
    pub insurance_reward: u64,
    pub cost_of_trades: u64,
    pub size: i64,
    pub remaining_liquidatee_balance: u64,
    pub remaining_liquidator_balance: u64,
    pub mark_price: u64,
    pub underlying_price: u64,
    pub liquidatee: solana_pubkey::Pubkey,
    pub liquidator: solana_pubkey::Pubkey,
    pub asset: Asset,
    pub liquidatee_margin_account: solana_pubkey::Pubkey,
    pub liquidator_margin_account: solana_pubkey::Pubkey,
}
