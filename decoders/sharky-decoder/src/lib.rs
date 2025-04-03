use solana_pubkey::Pubkey;

pub struct SharkyDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("SHARKobtfF1bHhxD2eqftjHBdVSCbKo9JtgK71FhELP");
