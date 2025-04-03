#![no_std]

use solana_pubkey::Pubkey;

pub struct SplAssociatedTokenAccountDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
