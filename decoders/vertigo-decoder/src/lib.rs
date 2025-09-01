use solana_pubkey::Pubkey;

pub struct VertigoDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("vrTGoBuy5rYSxAfV3jaRJWHH6nN9WK4NRExGxsk1bCJ");
