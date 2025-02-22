use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct SystemProgramDecoder;
pub mod accounts;
pub mod instructions;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("11111111111111111111111111111111");
