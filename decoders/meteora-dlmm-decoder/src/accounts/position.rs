use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xaabc8fe47a40f7d0")]
pub struct Position {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub liquidity_shares: [u64; 70],
    pub reward_infos: [UserRewardInfo; 70],
    pub fee_infos: [FeeInfo; 70],
    pub lower_bin_id: i32,
    pub upper_bin_id: i32,
    pub last_updated_at: i64,
    pub total_claimed_fee_x_amount: u64,
    pub total_claimed_fee_y_amount: u64,
    pub total_claimed_rewards: [u64; 2],
    pub reserved: [u8; 160],
}
