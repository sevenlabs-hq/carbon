use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct JupiterLimitOrder2Decoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X");
