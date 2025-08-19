use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d41bf195b1bfcc028")]
pub struct PlaceOrderEvent {
    pub fee: u64,
    pub oracle_price: u64,
    pub order_id: u128,
    pub expiry_ts: u64,
    pub asset: Asset,
    pub margin_account: solana_pubkey::Pubkey,
    pub client_order_id: u64,
    pub user: solana_pubkey::Pubkey,
}
