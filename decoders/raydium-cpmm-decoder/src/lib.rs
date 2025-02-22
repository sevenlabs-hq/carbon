use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct RaydiumCpmmDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
