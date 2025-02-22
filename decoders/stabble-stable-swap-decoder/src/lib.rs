use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct StableSwapDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ");
