use solana_pubkey::Pubkey;

pub struct BoopDecoder;

pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4");
