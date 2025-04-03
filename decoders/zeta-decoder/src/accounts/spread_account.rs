use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x3982fc88a7b12fa2")]
pub struct SpreadAccount {
    pub authority: solana_pubkey::Pubkey,
    pub nonce: u8,
    pub balance: u64,
    pub series_expiry: [u64; 5],
    pub series_expiry_padding: u64,
    pub positions: [Position; 46],
    pub positions_padding: [Position; 92],
    pub asset: Asset,
    pub padding: [u8; 262],
}
