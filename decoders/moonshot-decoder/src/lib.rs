#![no_std]
extern crate alloc;
use solana_pubkey::Pubkey;

pub struct MoonshotDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG");
