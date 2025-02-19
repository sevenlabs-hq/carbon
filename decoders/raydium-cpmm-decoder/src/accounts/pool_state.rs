use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xf7ede3f5d7c3de46")]
pub struct PoolState {
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_creator: solana_sdk::pubkey::Pubkey,
    pub token0_vault: solana_sdk::pubkey::Pubkey,
    pub token1_vault: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub token0_mint: solana_sdk::pubkey::Pubkey,
    pub token1_mint: solana_sdk::pubkey::Pubkey,
    pub token0_program: solana_sdk::pubkey::Pubkey,
    pub token1_program: solana_sdk::pubkey::Pubkey,
    pub observation_key: solana_sdk::pubkey::Pubkey,
    pub auth_bump: u8,
    pub status: u8,
    pub lp_mint_decimals: u8,
    pub mint0_decimals: u8,
    pub mint1_decimals: u8,
    pub lp_supply: u64,
    pub protocol_fees_token0: u64,
    pub protocol_fees_token1: u64,
    pub fund_fees_token0: u64,
    pub fund_fees_token1: u64,
    pub open_time: u64,
    pub padding: [u64; 32],
}
