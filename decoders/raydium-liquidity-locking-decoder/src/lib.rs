use solana_sdk::{pubkey, pubkey::Pubkey};

pub struct RaydiumLiquidityLockingDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub(crate) const PROGRAM_ID: Pubkey = pubkey!("LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE");
