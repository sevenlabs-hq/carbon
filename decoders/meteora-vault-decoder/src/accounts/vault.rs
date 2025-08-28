use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd308e82b02987577")]
pub struct Vault {
    pub enabled: u8,
    pub bumps: VaultBumps,
    pub total_amount: u64,
    pub token_vault: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub strategies: [solana_pubkey::Pubkey; 30],
    pub base: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub operator: solana_pubkey::Pubkey,
    pub locked_profit_tracker: LockedProfitTracker,
}
