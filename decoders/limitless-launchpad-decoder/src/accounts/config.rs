use crate::types::{FeeDistributionConfig, GraduationVenue};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

pub const CONFIG_DISCRIMINATOR: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Config {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub fee_splitter_program: Pubkey,
    pub bonding_curve_fee_bps: u16,
    pub bonding_platform_share_bps: u16,
    pub bonding_creator_share_bps: u16,
    pub bonding_rewards_share_bps: u16,
    pub standard_transfer_fee_bps: u16,
    pub shopify_transfer_fee_bps: u16,
    pub post_grad_creator_share_bps: u16,
    pub post_grad_platform_share_bps: u16,
    pub distribution: FeeDistributionConfig,
    pub graduation_sol_threshold: u64,
    pub bump: u8,
    pub premium_transfer_fee_bps: u16,
    pub trader_transfer_fee_bps: u16,
    pub community_transfer_fee_bps: u16,
    pub max_creator_first_buy_bps: u16,
    pub first_buy_max_pct_of_supply: u16,
    pub is_paused: bool,
    pub graduation_venue_default: GraduationVenue,
    pub meteora_pool_fee_bps: u16,
    pub meteora_graduation_enabled: bool,
    pub meteora_config: Pubkey,
    pub meteora_config_is_static: bool,
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub reserved: [u8; 200],
}

impl Config {
    pub const LEN: usize = 385;

    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() != Self::LEN || data[..8] != CONFIG_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }

    /// Returns the effective Meteora DAMM v2 pool fee in tenths of a bps (`1250` = `1.25%`).
    pub fn effective_meteora_pool_fee_tenths_of_bps(&self) -> u16 {
        if self.meteora_pool_fee_bps == 0 {
            1_250
        } else {
            self.meteora_pool_fee_bps
        }
    }
}
