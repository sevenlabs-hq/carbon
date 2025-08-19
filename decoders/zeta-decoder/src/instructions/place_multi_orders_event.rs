use {
    super::super::types::*,
    alloc::vec::Vec,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dee081207a23b6891")]
pub struct PlaceMultiOrdersEvent {
    pub oracle_price: u64,
    pub order_ids: Vec<u128>,
    pub expiry_tss: Vec<u64>,
    pub asset: Asset,
    pub margin_account: solana_pubkey::Pubkey,
    pub client_order_ids: Vec<u64>,
    pub user: solana_pubkey::Pubkey,
}
