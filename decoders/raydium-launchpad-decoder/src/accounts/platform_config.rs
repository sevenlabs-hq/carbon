use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa04e8000f853e6a0")]
pub struct PlatformConfig {
    pub epoch: u64,
    pub platform_fee_wallet: solana_pubkey::Pubkey,
    pub platform_nft_wallet: solana_pubkey::Pubkey,
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
    pub fee_rate: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub name: [u8; 64],
    #[serde(with = "serde_big_array::BigArray")]
    pub web: [u8; 256],
    #[serde(with = "serde_big_array::BigArray")]
    pub img: [u8; 256],
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 256],
}
