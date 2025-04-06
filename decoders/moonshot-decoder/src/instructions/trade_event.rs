use {
    super::super::types::*,
    alloc::string::String,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbddb7fd34ee661ee")]
pub struct TradeEvent {
    pub amount: u64,
    pub collateral_amount: u64,
    pub dex_fee: u64,
    pub helio_fee: u64,
    pub allocation: u64,
    pub curve: solana_pubkey::Pubkey,
    pub cost_token: solana_pubkey::Pubkey,
    pub sender: solana_pubkey::Pubkey,
    pub trade_type: TradeType,
    pub label: String,
}
