#![no_std]
extern crate alloc;

use solana_pubkey::Pubkey;
pub struct Token2022Decoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey = spl_token_2022::id();
