use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum LaunchType {
    Standard = 0,
    Shopify = 1,
    Premium = 2,
    RetiredTrader = 3,
    RetiredCommunity = 4,
}

impl BorshDeserialize for LaunchType {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        match u8::deserialize_reader(reader)? {
            0 => Ok(Self::Standard),
            1 => Ok(Self::Shopify),
            2 => Ok(Self::Premium),
            3 => Ok(Self::RetiredTrader),
            4 | 5 => Ok(Self::RetiredCommunity),
            other => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid LaunchType byte: {other}"),
            )),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum BadgeTier {
    Standard = 0,
    Bronze = 1,
    Silver = 2,
    Gold = 3,
    Platinum = 4,
    Diamond = 5,
    Obsidian = 6,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum BondingCurveType {
    ConstantProduct = 0,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum GraduationVenue {
    Raydium = 0,
    LimitlessSwap = 1,
    MeteoraDammV2 = 2,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct FeeDistributionConfig {
    pub treasury_share_bps: u16,
    pub nft_rewards_share_bps: u16,
    pub lp_incentive_share_bps: u16,
    pub ecosystem_share_bps: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LaunchCoinParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub token_on_curve: u64,
    pub sol_listed: u64,
    pub initial_sol_price: u64,
    pub launch_type: LaunchType,
    pub creator_fee_recipient: Pubkey,
    pub virtual_token_offset: u64,
    pub charity_wallet: Option<Pubkey>,
    pub charity_bps: u16,
    pub first_buy_sol_amount: Option<u64>,
    pub first_buy_min_tokens_out: u64,
    pub graduation_sol_threshold_override: Option<u64>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct CreateAndFirstBuyParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub token_on_curve: u64,
    pub sol_listed: u64,
    pub launch_type: LaunchType,
    pub genie_allocation_bps: u16,
    pub creator_fee_recipient: Pubkey,
    pub virtual_token_offset: u64,
    pub charity_wallet: Option<Pubkey>,
    pub charity_bps: u16,
    pub first_buy_sol_amount: u64,
    pub first_buy_min_tokens_out: u64,
    pub graduation_sol_threshold_override: Option<u64>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PdaVanityFee {
    pub recipient: Pubkey,
    pub lamports: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct BuyParams {
    pub sol_in: u64,
    pub min_tokens_out: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct SellParams {
    pub tokens_in: u64,
    pub min_sol_out: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct GraduateMeteoraParams {
    pub sqrt_price: u128,
    pub liquidity: u128,
}
