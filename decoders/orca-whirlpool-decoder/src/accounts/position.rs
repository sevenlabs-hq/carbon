 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xaabc8fe47a40f7d0")] 
pub struct Position { 
        pub whirlpool: solana_sdk::pubkey::Pubkey, 
        pub position_mint: solana_sdk::pubkey::Pubkey, 
        pub liquidity: u128, 
        pub tick_lower_index: i32, 
        pub tick_upper_index: i32, 
        pub fee_growth_checkpoint_a: u128, 
        pub fee_owed_a: u64, 
        pub fee_growth_checkpoint_b: u128, 
        pub fee_owed_b: u64, 
        pub reward_infos: [PositionRewardInfo; 3], 
}