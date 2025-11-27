use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c26fac72e9a20d8")]
pub struct PositionRequest {
    pub owner: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub open_time: i64,
    pub update_time: i64,
    pub size_usd_delta: u64,
    pub collateral_delta: u64,
    pub request_change: RequestChange,
    pub request_type: RequestType,
    pub side: Side,
    pub price_slippage: Option<u64>,
    pub jupiter_minimum_out: Option<u64>,
    pub pre_swap_amount: Option<u64>,
    pub trigger_price: Option<u64>,
    pub trigger_above_threshold: Option<bool>,
    pub entire_position: Option<bool>,
    pub executed: bool,
    pub counter: u64,
    pub bump: u8,
    pub referral: Option<solana_pubkey::Pubkey>,
}
