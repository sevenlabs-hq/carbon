use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct JupiterLimitOrderDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu");
