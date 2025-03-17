use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct StakeProgramDecoder;

pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey = pubkey!("Stake11111111111111111111111111111111111111");
