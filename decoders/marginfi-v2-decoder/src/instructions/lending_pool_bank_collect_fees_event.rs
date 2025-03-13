use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d657761faa9af9cfd")]
pub struct LendingPoolBankCollectFeesEvent {
    pub header: GroupEventHeader,
    pub bank: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub group_fees_collected: f64,
    pub group_fees_outstanding: f64,
    pub insurance_fees_collected: f64,
    pub insurance_fees_outstanding: f64,
}
