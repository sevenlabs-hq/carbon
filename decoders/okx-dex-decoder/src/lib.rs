use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct OkxDexDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma");
