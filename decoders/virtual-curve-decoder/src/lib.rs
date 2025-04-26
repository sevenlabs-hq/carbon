#![no_std]

extern crate alloc;

use solana_pubkey::Pubkey;

pub struct VirtualCurveDecoder;

pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN");
