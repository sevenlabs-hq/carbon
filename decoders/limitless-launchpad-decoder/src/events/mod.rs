use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

pub const COIN_LAUNCHED_DISCRIMINATOR: [u8; 8] = [169, 65, 49, 61, 3, 18, 32, 179];
pub const BOUGHT_DISCRIMINATOR: [u8; 8] = [193, 56, 215, 24, 156, 76, 42, 104];
pub const SOLD_DISCRIMINATOR: [u8; 8] = [205, 203, 210, 202, 96, 11, 192, 10];
pub const GRADUATED_DISCRIMINATOR: [u8; 8] = [51, 241, 66, 50, 140, 245, 156, 192];
pub const METEORA_GRADUATED_DISCRIMINATOR: [u8; 8] = [22, 30, 196, 152, 16, 27, 229, 231];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CoinLaunched {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub launch_type: u8,
    pub transfer_fee_bps: u16,
    pub total_supply: u64,
    pub token_on_curve: u64,
    pub sol_listed: u64,
    pub preferred_swap_venue: Pubkey,
    pub virtual_token_offset: u64,
    pub decimals: u8,
    pub charity_wallet: Pubkey,
    pub charity_bps: u16,
    pub first_buy_sol_amount: u64,
    pub graduation_sol_threshold: u64,
    pub graduation_venue: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub timestamp: i64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Bought {
    pub mint: Pubkey,
    pub buyer: Pubkey,
    pub sol_in: u64,
    pub tokens_out: u64,
    pub platform_fee: u64,
    pub creator_fee: u64,
    pub rewards_fee: u64,
    pub cumulative_volume: u128,
    pub timestamp: i64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Sold {
    pub mint: Pubkey,
    pub seller: Pubkey,
    pub tokens_in: u64,
    pub sol_out: u64,
    pub platform_fee: u64,
    pub creator_fee: u64,
    pub rewards_fee: u64,
    pub cumulative_volume: u128,
    pub timestamp: i64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Graduated {
    pub mint: Pubkey,
    pub venue: Pubkey,
    pub lp_mint: Pubkey,
    pub sol_deposited: u64,
    pub tokens_deposited: u64,
    pub timestamp: i64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MeteoraGraduated {
    pub token_dust_returned: u64,
    pub mint: Pubkey,
    pub pool: Pubkey,
    pub creator: Pubkey,
    pub treasury: Pubkey,
    pub creator_position: Pubkey,
    pub partner_position: Pubkey,
    pub sol_deposited: u64,
    pub tokens_deposited: u64,
    pub pool_fee_bps: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "data"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimitlessLaunchpadEvent {
    CoinLaunched(Box<CoinLaunched>),
    Bought(Bought),
    Sold(Sold),
    Graduated(Graduated),
    MeteoraGraduated(Box<MeteoraGraduated>),
}

impl LimitlessLaunchpadEvent {
    /// Decodes raw bytes emitted by Anchor's `Program data: <base64>` log line.
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }
        let disc = &data[..8];
        let mut slice = &data[8..];
        if disc == COIN_LAUNCHED_DISCRIMINATOR {
            return CoinLaunched::deserialize(&mut slice)
                .ok()
                .map(|ev| Self::CoinLaunched(Box::new(ev)));
        }
        if disc == BOUGHT_DISCRIMINATOR {
            return Bought::deserialize(&mut slice).ok().map(Self::Bought);
        }
        if disc == SOLD_DISCRIMINATOR {
            return Sold::deserialize(&mut slice).ok().map(Self::Sold);
        }
        if disc == GRADUATED_DISCRIMINATOR {
            return Graduated::deserialize(&mut slice).ok().map(Self::Graduated);
        }
        if disc == METEORA_GRADUATED_DISCRIMINATOR {
            return MeteoraGraduated::deserialize(&mut slice)
                .ok()
                .map(|ev| Self::MeteoraGraduated(Box::new(ev)));
        }
        None
    }
}
