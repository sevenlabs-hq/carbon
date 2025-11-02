use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PersonalPositionState {
    pub bump: [u8; 1],
    pub nft_mint: solana_pubkey::Pubkey,
    pub pool_id: solana_pubkey::Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub fee_growth_inside_0_last_x64: u128,
    pub fee_growth_inside_1_last_x64: u128,
    pub token_fees_owed_0: u64,
    pub token_fees_owed_1: u64,
    pub reward_infos: [PositionRewardInfo; 3],
    pub recent_epoch: u64,
    pub padding: [u64; 7],
}
