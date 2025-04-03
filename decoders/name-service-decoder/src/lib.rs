use solana_pubkey::Pubkey;

pub struct NameDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");
