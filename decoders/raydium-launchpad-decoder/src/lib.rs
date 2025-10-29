#![no_std]
extern crate alloc;

use solana_pubkey::Pubkey;

pub struct RaydiumLaunchpadDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj");

pub const DEVNET_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("DRay6fNdQ5J82H7xV6uq2aV3mNrUZ1J4PgSKsWgptcm6");
