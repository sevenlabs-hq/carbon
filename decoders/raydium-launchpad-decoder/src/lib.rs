#![no_std]
extern crate alloc;

use solana_pubkey::Pubkey;

pub struct RaydiumLaunchpadDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj");
