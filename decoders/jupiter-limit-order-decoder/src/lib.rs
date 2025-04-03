use solana_pubkey::Pubkey;

pub struct JupiterLimitOrderDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu");
