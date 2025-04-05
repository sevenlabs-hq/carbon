#![no_std]

extern crate alloc;

use solana_pubkey::Pubkey;

pub struct RaydiumClmmDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");
