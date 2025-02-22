use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct Token2022Decoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
