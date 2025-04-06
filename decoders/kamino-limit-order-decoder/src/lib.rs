use solana_pubkey::Pubkey;

pub struct KaminoLimitOrderDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("LiMoM9rMhrdYrfzUCxQppvxCSG1FcrUK9G8uLq4A1GF");
