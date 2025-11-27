use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd8926b5e684bb6b1")]
pub struct State {
    pub admin: solana_pubkey::Pubkey,
    pub whitelist_mint: solana_pubkey::Pubkey,
    pub discount_mint: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub srm_vault: solana_pubkey::Pubkey,
    pub perp_fee_structure: FeeStructure,
    pub spot_fee_structure: FeeStructure,
    pub oracle_guard_rails: OracleGuardRails,
    pub number_of_authorities: u64,
    pub number_of_sub_accounts: u64,
    pub lp_cooldown_time: u64,
    pub liquidation_margin_buffer_ratio: u32,
    pub settlement_duration: u16,
    pub number_of_markets: u16,
    pub number_of_spot_markets: u16,
    pub signer_nonce: u8,
    pub min_perp_auction_duration: u8,
    pub default_market_order_time_in_force: u8,
    pub default_spot_auction_duration: u8,
    pub exchange_status: u8,
    pub liquidation_duration: u8,
    pub initial_pct_to_liquidate: u16,
    pub max_number_of_sub_accounts: u16,
    pub max_initialize_user_fee: u16,
    pub padding: [u8; 10],
}
