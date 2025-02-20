use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xdbbed53700e3c69a")]
pub struct Market {
    pub bump: u8,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub padding1: [u8; 5],
    pub market_authority: solana_sdk::pubkey::Pubkey,
    pub time_expiry: i64,
    pub collect_fee_admin: solana_sdk::pubkey::Pubkey,
    pub open_orders_admin: NonZeroPubkeyOption,
    pub consume_events_admin: NonZeroPubkeyOption,
    pub close_market_admin: NonZeroPubkeyOption,
    pub name: [u8; 16],
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub event_heap: solana_sdk::pubkey::Pubkey,
    pub oracle_a: NonZeroPubkeyOption,
    pub oracle_b: NonZeroPubkeyOption,
    pub oracle_config: OracleConfig,
    pub quote_lot_size: i64,
    pub base_lot_size: i64,
    pub seq_num: u64,
    pub registration_time: i64,
    pub maker_fee: i64,
    pub taker_fee: i64,
    pub fees_accrued: u128,
    pub fees_to_referrers: u128,
    pub referrer_rebates_accrued: u64,
    pub fees_available: u64,
    pub maker_volume: u128,
    pub taker_volume_wo_oo: u128,
    pub base_mint: solana_sdk::pubkey::Pubkey,
    pub quote_mint: solana_sdk::pubkey::Pubkey,
    pub market_base_vault: solana_sdk::pubkey::Pubkey,
    pub base_deposit_total: u64,
    pub market_quote_vault: solana_sdk::pubkey::Pubkey,
    pub quote_deposit_total: u64,
    pub reserved: [u8; 128],
}
