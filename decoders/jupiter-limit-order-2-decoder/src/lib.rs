use solana_pubkey::Pubkey;

pub struct JupiterLimitOrder2Decoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X");
