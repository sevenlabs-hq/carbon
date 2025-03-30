#![no_std]

extern crate alloc;

pub struct TokenProgramDecoder;

pub mod accounts;
pub mod instructions;
pub mod types;

#[cfg(feature = "postgres")]
pub use postgres_client::*;