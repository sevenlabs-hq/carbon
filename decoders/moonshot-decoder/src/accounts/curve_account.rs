use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug, PartialEq, Default)]
#[carbon(discriminator = "0x085b531c84d8f816")]
pub struct CurveAccount {
    pub total_supply: u64,
    pub curve_amount: u64,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub decimals: u8,
    pub collateral_currency: Currency,
    pub curve_type: CurveType,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub migration_fee: u64,
    pub coef_b: u32,
    pub bump: u8,
    pub migration_target: MigrationTarget,
    pub _reserved1: [u8; 32],
    pub _reserved2: [u8; 32],
    pub _reserved3: [u8; 32],
    pub _reserved4: [u8; 32],
    pub _reserved5: [u8; 32],
    pub _reserved6: [u8; 32],
    pub _reserved7: [u8; 32],
    pub _reserved8: [u8; 32],
    pub _reserved9: [u8; 32],
    pub _reserved10: [u8; 32],
    pub _reserved11: [u8; 7],
}
