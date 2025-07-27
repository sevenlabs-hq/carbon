use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x085b531c84d8f816")]
pub struct CurveAccount {
    pub total_supply: u64,
    pub curve_amount: u64,
    pub mint: solana_pubkey::Pubkey,
    pub decimals: u8,
    pub collateral_currency: Currency,
    pub curve_type: CurveType,
    pub marketcap_threshold: u64,
    pub marketcap_currency: Currency,
    pub migration_fee: u64,
    pub coef_b: u32,
    pub bump: u8,
    pub migration_target: MigrationTarget,
    #[serde(with = "serde_big_array::BigArray")]
    pub _reserved: [u8; 327],
}

impl Default for CurveAccount {
    fn default() -> Self {
        Self {
            total_supply: 0,
            curve_amount: 0,
            mint: solana_pubkey::Pubkey::default(),
            decimals: 0,
            collateral_currency: Currency::default(),
            curve_type: CurveType::default(),
            marketcap_threshold: 0,
            marketcap_currency: Currency::default(),
            migration_fee: 0,
            coef_b: 0,
            bump: 0,
            migration_target: MigrationTarget::default(),
            _reserved: [0; 327],
        }
    }
}
