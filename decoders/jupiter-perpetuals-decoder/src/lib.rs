use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct PerpetualsDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu");
