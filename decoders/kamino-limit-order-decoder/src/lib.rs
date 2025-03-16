use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct KaminoLimitOrderDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey = pubkey!("LiMoM9rMhrdYrfzUCxQppvxCSG1FcrUK9G8uLq4A1GF");
