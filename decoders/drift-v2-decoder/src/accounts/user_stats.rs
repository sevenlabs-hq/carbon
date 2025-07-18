use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb0df881b7a4f20e3")]
pub struct UserStats {
    pub authority: solana_pubkey::Pubkey,
    pub referrer: solana_pubkey::Pubkey,
    pub fees: UserFees,
    pub next_epoch_ts: i64,
    pub maker_volume30d: u64,
    pub taker_volume30d: u64,
    pub filler_volume30d: u64,
    pub last_maker_volume30d_ts: i64,
    pub last_taker_volume30d_ts: i64,
    pub last_filler_volume30d_ts: i64,
    pub if_staked_quote_asset_amount: u64,
    pub number_of_sub_accounts: u16,
    pub number_of_sub_accounts_created: u16,
    pub referrer_status: u8,
    pub disable_update_perp_bid_ask_twap: bool,
    pub padding1: [u8; 1],
    pub fuel_overflow_status: u8,
    pub fuel_insurance: u32,
    pub fuel_deposits: u32,
    pub fuel_borrows: u32,
    pub fuel_positions: u32,
    pub fuel_taker: u32,
    pub fuel_maker: u32,
    pub if_staked_gov_token_amount: u64,
    pub last_fuel_if_bonus_update_ts: u32,
    pub padding: [u8; 12],
}
