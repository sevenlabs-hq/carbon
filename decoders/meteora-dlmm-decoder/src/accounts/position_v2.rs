use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x75b0d4c7f5b485b6")]
pub struct PositionV2 {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub liquidity_shares: [u128; 70],
    pub reward_infos: [UserRewardInfo; 70],
    pub fee_infos: [FeeInfo; 70],
    pub lower_bin_id: i32,
    pub upper_bin_id: i32,
    pub last_updated_at: i64,
    pub total_claimed_fee_x_amount: u64,
    pub total_claimed_fee_y_amount: u64,
    pub total_claimed_rewards: [u64; 2],
    pub operator: solana_sdk::pubkey::Pubkey,
    pub lock_release_point: u64,
    pub padding0: u8,
    pub fee_owner: solana_sdk::pubkey::Pubkey,
    pub reserved: [u8; 87],
}
