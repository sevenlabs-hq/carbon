
use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1da6a0f99ab72717f2")]
pub struct LendingAccountLiquidateEvent{
    pub header: AccountEventHeader,
    pub liquidatee_marginfi_account: solana_sdk::pubkey::Pubkey,
    pub liquidatee_marginfi_account_authority: solana_sdk::pubkey::Pubkey,
    pub asset_bank: solana_sdk::pubkey::Pubkey,
    pub asset_mint: solana_sdk::pubkey::Pubkey,
    pub liability_bank: solana_sdk::pubkey::Pubkey,
    pub liability_mint: solana_sdk::pubkey::Pubkey,
    pub liquidatee_pre_health: f64,
    pub liquidatee_post_health: f64,
    pub pre_balances: LiquidationBalances,
    pub post_balances: LiquidationBalances,
}
