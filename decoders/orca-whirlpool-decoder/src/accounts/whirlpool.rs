 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x3f95d10ce1806309")] 
pub struct Whirlpool { 
        pub whirlpools_config: solana_sdk::pubkey::Pubkey, 
        pub whirlpool_bump: [u8; 1], 
        pub tick_spacing: u16, 
        pub tick_spacing_seed: [u8; 2], 
        pub fee_rate: u16, 
        pub protocol_fee_rate: u16, 
        pub liquidity: u128, 
        pub sqrt_price: u128, 
        pub tick_current_index: i32, 
        pub protocol_fee_owed_a: u64, 
        pub protocol_fee_owed_b: u64, 
        pub token_mint_a: solana_sdk::pubkey::Pubkey, 
        pub token_vault_a: solana_sdk::pubkey::Pubkey, 
        pub fee_growth_global_a: u128, 
        pub token_mint_b: solana_sdk::pubkey::Pubkey, 
        pub token_vault_b: solana_sdk::pubkey::Pubkey, 
        pub fee_growth_global_b: u128, 
        pub reward_last_updated_timestamp: u64, 
        pub reward_infos: [WhirlpoolRewardInfo; 3], 
}