use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct MoonshotDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG");
