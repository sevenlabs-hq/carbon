use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct SharkyDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("SHARKobtfF1bHhxD2eqftjHBdVSCbKo9JtgK71FhELP");
