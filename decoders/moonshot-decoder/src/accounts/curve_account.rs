use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
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
}
