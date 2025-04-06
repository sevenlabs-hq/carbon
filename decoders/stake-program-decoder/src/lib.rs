#![no_std]

extern crate alloc;
use solana_pubkey::Pubkey;

pub struct StakeProgramDecoder;

pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("Stake11111111111111111111111111111111111111");
