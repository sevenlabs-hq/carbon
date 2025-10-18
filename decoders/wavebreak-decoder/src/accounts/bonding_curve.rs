use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct BondingCurve {
    pub discriminator: AccountDiscriminator,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub retain_mint_authority: bool,
    pub buy_requires_permission: bool,
    pub buy_permission_bitmap: [u8; 32],
    pub sell_requires_permission: bool,
    pub sell_permission_bitmap: [u8; 32],
    pub quote_fee_bps: u16,
    pub base_fee_bps: u16,
    pub control_points: [u16; 4],
    pub start_price: u128,
    pub end_price: u128,
    pub quote_amount: u64,
    pub base_amount: u64,
    pub launch_slot: u64,
    pub creator_reward: u64,
    pub graduation_target: u64,
    pub graduation_slot: u64,
    pub graduation_reward: u64,
    pub max_buy_amount: u64,
    pub max_sell_amount: u64,
    pub swap_fee_bps: u16,
    pub base_allocation_bps: u16,
    pub graduation_methods: [GraduationMethodData; 8],
    pub min_reserve_bps: u16,
    pub padding1: [u8; 2],
    pub preminted_supply: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding2: [u8; 728],
}
