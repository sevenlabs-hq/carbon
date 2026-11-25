use crate::types::{BadgeTier, BondingCurveType, GraduationVenue, LaunchType};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

pub const LAUNCHED_COIN_DISCRIMINATOR: [u8; 8] = [148, 134, 217, 15, 190, 137, 244, 224];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LaunchedCoin {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub creator_fee_recipient: Pubkey,
    pub launch_type: LaunchType,
    pub badge_tier: BadgeTier,
    pub bonding_curve_type: BondingCurveType,
    pub transfer_fee_bps: u16,
    pub decimals: u8,
    pub total_supply: u64,
    pub token_on_curve: u64,
    pub sol_listed: u64,
    pub tokens_remaining_on_curve: u64,
    pub sol_reserves: u64,
    pub cumulative_sol_volume: u128,
    pub cumulative_token_volume: u128,
    pub graduated: bool,
    pub preferred_swap_venue: Pubkey,
    pub graduation_sol_threshold: u64,
    pub bump: u8,
    pub virtual_token_offset: u64,
    pub charity_wallet: Pubkey,
    pub charity_bps: u16,
    pub version: u8,
    pub graduation_venue: GraduationVenue,
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub reserved: [u8; 76],
}

impl LaunchedCoin {
    pub const LEN: usize = 344;

    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() != Self::LEN || data[..8] != LAUNCHED_COIN_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }

    pub fn virtual_sol_lamports(&self) -> Option<u64> {
        self.sol_listed.checked_add(self.sol_reserves)
    }

    pub fn virtual_token_units(&self) -> Option<u64> {
        self.virtual_token_offset
            .checked_add(self.tokens_remaining_on_curve)
    }

    pub fn meteora_prepared(&self) -> bool {
        !self.graduated
            && self.graduation_venue == GraduationVenue::MeteoraDammV2
            && self.reserved[0] == 1
    }

    pub fn curve_complete(&self) -> bool {
        self.graduated || self.meteora_prepared() || self.reserved[31] != 0
    }
}
