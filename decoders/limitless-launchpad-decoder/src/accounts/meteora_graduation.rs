use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

pub const METEORA_GRADUATION_DISCRIMINATOR: [u8; 8] = [207, 89, 172, 119, 25, 245, 163, 116];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MeteoraGraduation {
    pub mint: Pubkey,
    pub pool: Pubkey,
    pub creator_fee_recipient: Pubkey,
    pub treasury: Pubkey,
    pub creator_position: Pubkey,
    pub partner_position: Pubkey,
    pub creator_share_bps: u16,
    pub platform_share_bps: u16,
    pub pool_fee_bps: u16,
    pub creator_locked_liquidity: u128,
    pub partner_locked_liquidity: u128,
    pub sol_deposited: u64,
    pub tokens_deposited: u64,
    pub bump: u8,
}

impl MeteoraGraduation {
    pub const LEN: usize = 255;

    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() != Self::LEN || data[..8] != METEORA_GRADUATION_DISCRIMINATOR {
            return None;
        }
        let mut slice = &data[8..];
        Self::deserialize(&mut slice).ok()
    }
}
