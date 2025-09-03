use solana_pubkey::Pubkey;

pub struct PumpFeesDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ");
