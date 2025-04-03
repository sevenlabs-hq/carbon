use solana_pubkey::Pubkey;

pub struct ZetaDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("ZETAxsqBRek56DhiGXrn75yj2NHU3aYUnxvHXpkf3aD");
