use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf25e8e8323f4931c")]
pub struct CrossMarginAccount {
    pub authority: solana_pubkey::Pubkey,
    pub delegated_pubkey: solana_pubkey::Pubkey,
    pub balance: u64,
    pub subaccount_index: u8,
    pub nonce: u8,
    pub force_cancel_flag: bool,
    pub account_type: MarginAccountType,
    pub open_orders_nonces: [u8; 25],
    pub open_orders_nonces_padding: [u8; 0],
    pub rebalance_amount: i64,
    pub last_funding_deltas: [AnchorDecimal; 25],
    pub last_funding_deltas_padding: [AnchorDecimal; 0],
    pub product_ledgers: [ProductLedger; 25],
    pub product_ledgers_padding: [ProductLedger; 0],
    pub trigger_order_bits: u128,
    pub rebate_rebalance_amount: u64,
    pub potential_order_loss: [u64; 25],
    pub potential_order_loss_padding: [u64; 0],
    #[serde(with = "serde_big_array::BigArray")]
    pub padding: [u8; 1776],
}
