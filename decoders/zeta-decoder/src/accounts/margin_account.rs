use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x85dcadd5b3d32bee")]
pub struct MarginAccount {
    pub authority: solana_pubkey::Pubkey,
    pub nonce: u8,
    pub balance: u64,
    pub force_cancel_flag: bool,
    #[serde(with = "serde_big_array::BigArray")]
    pub open_orders_nonce: [u8; 138],
    pub series_expiry: [u64; 5],
    pub series_expiry_padding: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub product_ledgers: [ProductLedger; 46],
    #[serde(with = "serde_big_array::BigArray")]
    pub product_ledgers_padding: [ProductLedger; 91],
    pub perp_product_ledger: ProductLedger,
    pub rebalance_amount: i64,
    pub asset: Asset,
    pub account_type: MarginAccountType,
    pub last_funding_delta: AnchorDecimal,
    pub delegated_pubkey: solana_pubkey::Pubkey,
    pub rebate_rebalance_amount: u64,
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 330],
}
