use solana_pubkey::Pubkey;

pub struct PerpetualsDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu");
