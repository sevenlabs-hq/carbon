use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct MintConfig {
    pub discriminator: AccountDiscriminator,
    pub instruction_discriminator: u8,
    pub quote_mint: solana_pubkey::Pubkey,
    pub create_requires_permission: bool,
    pub create_permission_bitmap: [u8; 32],
    pub default_buy_requires_permission: bool,
    pub default_buy_permission_bitmap: [u8; 32],
    pub default_sell_requires_permission: bool,
    pub default_sell_permission_bitmap: [u8; 32],
    pub padding1: [u8; 3],
    pub default_creator_reward: u64,
    pub default_graduation_reward: u64,
    pub default_graduation_target: u64,
    pub default_max_buy_amount: u64,
    pub default_max_sell_amount: u64,
    pub default_start_price: u128,
    pub default_end_price: u128,
    pub default_control_points: [u16; 4],
    pub default_swap_fee_bps: u16,
    pub default_quote_fee_bps: u16,
    pub default_base_fee_bps: u16,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding2: [u8; 1826],
}
