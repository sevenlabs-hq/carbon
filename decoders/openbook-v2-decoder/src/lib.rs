use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct OpenbookV2Decoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb");
