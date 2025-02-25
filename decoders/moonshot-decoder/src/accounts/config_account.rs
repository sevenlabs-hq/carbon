use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug, PartialEq, Default)]
#[carbon(discriminator = "0xbdff6146babd1866")]
pub struct ConfigAccount {
    pub migration_authority: solana_sdk::pubkey::Pubkey,
    pub backend_authority: solana_sdk::pubkey::Pubkey,
    pub config_authority: solana_sdk::pubkey::Pubkey,
    pub helio_fee: solana_sdk::pubkey::Pubkey,
    pub dex_fee: solana_sdk::pubkey::Pubkey,
    pub fee_bps: u16,
    pub dex_fee_share: u8,
    pub migration_fee: u64,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub min_supported_decimal_places: u8,
    pub max_supported_decimal_places: u8,
    pub min_supported_token_supply: u64,
    pub max_supported_token_supply: u64,
    pub bump: u8,
    pub coef_b: u32,
    pub _reserved1: [u8; 32],
    pub _reserved2: [u8; 32],
    pub _reserved3: [u8; 32],
    pub _reserved4: [u8; 32],
    pub _reserved5: [u8; 32],
    pub _reserved6: [u8; 32],
}
